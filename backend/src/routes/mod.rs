pub mod project;
pub mod user;

use actix_session::Session;
use actix_web::{HttpResponse, Responder};
pub use project::*;
pub use user::*;

pub async fn health_check(session: Session) -> impl Responder {
    let test = session.entries();
    tracing::info!("{:?}", test);
    HttpResponse::Ok()
}
