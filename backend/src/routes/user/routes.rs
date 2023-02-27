use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, HttpResponseBuilder, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use secrecy::ExposeSecret;
use sqlx::SqlitePool;
use super::types::*;

#[post("/delete-user")]
#[tracing::instrument(name = "Deleting user.", skip(pool, session))]
async fn delete_user(pool: web::Data<SqlitePool>, session: Session) -> impl Responder {
    match get_user_id(&session).await {
        Ok(id) => match delete_user_from_db(&pool, id).await {
            Ok(_) => {
                session.purge();
                HttpResponse::NoContent().finish()
            }
            Err(e) => e.into(),
        },
        Err(e) => e.into(),
    }
}

#[post("/log-in")]
#[tracing::instrument(
    name = "Logging in user.",
    skip(body, pool, session),
    fields(
        %body.username
    )
)]
async fn login(
    body: web::Json<UserRequest>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    match check_user_password(&body, &pool).await {
        Ok(user_id) => create_session(HttpResponse::Ok(), &session, user_id),
        Err(e) => {
            tracing::error!("Error recieved during log in: {:?}", e);
            e.into()
        }
    }
}

#[post("/log-out")]
#[tracing::instrument(name = "Logging out user.", skip(session))]
async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::NoContent().finish()
}

#[post("/sign-up")]
#[tracing::instrument(
    name = "Signing up new user.",
    skip(body, pool, session),
    fields(
        %body.username
    )
)]
async fn signup(
    body: web::Json<UserRequest>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    match sign_up_user(&body, &pool).await {
        Ok(user_id) => create_session(HttpResponse::Created(), &session, user_id),
        Err(e) => {
            tracing::error!("Error recieved during sign up: {:?}", e);
            e.into()
        }
    }
}

#[get("/user-status")]
#[tracing::instrument(name = "Checking user status.", skip(session))]
async fn user_status(session: Session) -> impl Responder {
    tracing::info!("Cookies recieved: {}", session.entries().capacity());
    let body = UserStatus {
        is_logged_in: !session.entries().is_empty(),
    };
    HttpResponse::Ok().json(body)
}

async fn delete_user_from_db(pool: &SqlitePool, user_id: i64) -> Result<(), UserError> {
    sqlx::query!(r#"DELETE FROM user WHERE id=?"#, user_id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn get_user_id(session: &Session) -> Result<i64, UserError> {
    let maybe_user_id = session.get::<i64>("user_id")?;
    let id = maybe_user_id.ok_or_else(|| {
        UserError::SessionGetError("User ID not found within session".to_string())
    })?;
    Ok(id)
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
    session.renew();
    match session.insert("user_id", &user_id) {
        Ok(_) => code,
        Err(_) => HttpResponse::InternalServerError(),
    }
    .finish()
}
