use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder, HttpResponseBuilder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Debug)]
enum UserError {
    PasswordHashError(argon2::password_hash::Error),
    SqlxError(sqlx::Error),
}

impl From<argon2::password_hash::Error> for UserError {
    fn from(e: argon2::password_hash::Error) -> Self {
        Self::PasswordHashError(e)
    }
}

impl From<sqlx::Error> for UserError {
    fn from(e: sqlx::Error) -> Self {
        Self::SqlxError(e)
    }
}

#[derive(Deserialize)]
struct UserRequest {
    username: String,
    password: Secret<String>,
}

async fn check_user_password(body: &UserRequest, pool: &SqlitePool) -> Result<i64, UserError> {
    let user = sqlx::query!(
        r#"SELECT id, password_hash FROM user WHERE name=?"#,
        body.username
    )
    .fetch_one(pool)
    .await?;

    let parsed_hash = PasswordHash::new(&user.password_hash)?;
    Argon2::default().verify_password(body.password.expose_secret().as_bytes(), &parsed_hash)?;
    Ok(user.id)
}

async fn sign_up_user(body: &UserRequest, pool: &SqlitePool) -> Result<i64, UserError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(body.password.expose_secret().as_bytes(), &salt)?
        .to_string();

    let user_id = sqlx::query!(
        r#"INSERT INTO user (name, password_hash) VALUES (?,?) RETURNING id"#,
        body.username,
        password_hash
    )
    .fetch_one(pool)
    .await?
    .id;

    Ok(user_id)
}

fn create_session(code: HttpResponseBuilder, session: &Session, user_id: i64) -> HttpResponse {
    match session.insert("user_id", &user_id) {
            Ok(_) => {
                session.renew();
                code
            }
            Err(_) => HttpResponse::InternalServerError(),
        }.into()
}

#[post("/log-in")]
async fn login(
    body: web::Json<UserRequest>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    match check_user_password(&body, &pool).await {
        Ok(user_id) => create_session(HttpResponse::Ok(), &session, user_id),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[post("/sign-up")]
async fn signup(
    body: web::Json<UserRequest>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    match sign_up_user(&body, &pool).await {
        Ok(user_id) => create_session(HttpResponse::Created(), &session, user_id),
        // TODO: more bespoke responses
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(signup);
}
