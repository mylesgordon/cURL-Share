use actix_session::SessionGetError;
use actix_web::HttpResponse;
use secrecy::Secret;
use serde::{Deserialize, Serialize};
use sqlx::error::DatabaseError;

#[derive(Debug)]
pub enum UserError {
    InvalidPassword(argon2::password_hash::Error),
    PasswordHashError(argon2::password_hash::Error),
    SessionGetError(String),
    SqlxDatabaseError(Box<dyn DatabaseError>),
    SqlxError(sqlx::Error),
    UserAlreadyExists,
    UserNotFound(sqlx::Error),
}

impl From<argon2::password_hash::Error> for UserError {
    fn from(e: argon2::password_hash::Error) -> Self {
        match e {
            argon2::password_hash::Error::Password => Self::InvalidPassword(e),
            _ => Self::PasswordHashError(e),
        }
    }
}

impl From<SessionGetError> for UserError {
    fn from(e: SessionGetError) -> Self {
        UserError::SessionGetError(e.to_string())
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
                        _ => Self::SqlxDatabaseError(database_err),
                    }
                } else {
                    Self::SqlxDatabaseError(database_err)
                }
            }
            _ => Self::SqlxError(e),
        }
    }
}

impl From<UserError> for HttpResponse {
    fn from(e: UserError) -> Self {
        match e {
            UserError::InvalidPassword(_) | UserError::SessionGetError(_) => {
                HttpResponse::Unauthorized().finish()
            }
            UserError::UserAlreadyExists => HttpResponse::Conflict().finish(),
            UserError::UserNotFound(_) => HttpResponse::Unauthorized().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[derive(Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: Secret<String>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UserStatus {
    pub is_logged_in: bool,
}
