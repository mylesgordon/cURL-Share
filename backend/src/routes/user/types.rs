use actix_web::HttpResponse;
use secrecy::Secret;
use serde::Deserialize;

#[derive(Debug)]
pub enum UserError {
    PasswordHashError(argon2::password_hash::Error),
    SqlxDatabaseError,
    SqlxError(sqlx::Error),
    UserAlreadyExists,
    UserNotFound(sqlx::Error),
}

impl From<argon2::password_hash::Error> for UserError {
    fn from(e: argon2::password_hash::Error) -> Self {
        Self::PasswordHashError(e)
    }
}

impl From<sqlx::Error> for UserError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Self::UserNotFound(e),
            sqlx::Error::Database(database_err) => {
                if let Some(database_err_code) = database_err.code() {
                    match database_err_code.as_ref() {
                        "2067" => Self::UserAlreadyExists,
                        _ => Self::SqlxDatabaseError,
                    }
                } else {
                    Self::SqlxDatabaseError
                }
            }
            _ => Self::SqlxError(e),
        }
    }
}

impl From<UserError> for HttpResponse {
    fn from(e: UserError) -> Self {
        match e {
            UserError::UserAlreadyExists => HttpResponse::Conflict().into(),
            UserError::UserNotFound(_) => HttpResponse::Unauthorized().into(),
            _ => HttpResponse::InternalServerError().into(),
        }
    }
}

#[derive(Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: Secret<String>,
}
