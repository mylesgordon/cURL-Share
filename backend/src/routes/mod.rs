pub mod project;
pub mod user;

use actix_web::{HttpResponse, Responder};
pub use project::*;
pub use user::*;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
