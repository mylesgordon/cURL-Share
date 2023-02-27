use actix_session::SessionGetError;
use serde::{Deserialize, Serialize};

use crate::routes::types::UserError;

#[derive(Debug)]
pub enum ProjectError {
    SessionGetError(String),
    SqlxError(sqlx::Error),
    Unauthorised(String),
    UserError(UserError),
}

impl From<SessionGetError> for ProjectError {
    fn from(e: SessionGetError) -> Self {
        match e {
            // TODO: improve
            _ => ProjectError::SessionGetError(e.to_string()),
        }
    }
}

impl From<sqlx::Error> for ProjectError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            // TODO: improve
            _ => ProjectError::SqlxError(e),
        }
    }
}

impl From<UserError> for ProjectError {
    fn from(e: UserError) -> Self {
        match e {
            // TODO: improve
            UserError::SessionGetError(e) => ProjectError::SessionGetError(e),
            _ => ProjectError::UserError(e),
        }
    }
}

#[derive(Serialize)]
pub struct ProjectId {
    pub id: i64,
}

#[derive(Debug, Default, Deserialize)]
pub struct ProjectParams {
    pub search: String,
    pub labels: Vec<String>,
}
