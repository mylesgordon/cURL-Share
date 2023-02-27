mod routes;
pub mod types;

use actix_web::web;
use routes::*;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(delete_user)
        .service(login)
        .service(logout)
        .service(signup)
        .service(user_status);
}
