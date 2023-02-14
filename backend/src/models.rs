use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Status {
    pub status: String,
}

pub struct CurlGroup {
    pub id: i64,
    pub curls: String,
    pub description: String,
    pub labels: String,
    pub name: String,
    pub project_id: i64
}

pub struct Project {
    pub id: i64,
    pub admin_user_id: i64,
    pub environments: String,
    pub description: String,
    pub name: String,
    pub visibility: String, // TODO: enum
}

pub struct ProjectCollaborators {
    pub project_id: i64,
    pub user_id: i64
}

pub struct User {
    pub id: i64,
    pub name: String,
    pub password_hash: String
}