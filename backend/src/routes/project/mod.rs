mod routes;
pub mod types;
use actix_web::web;
use routes::*;

pub fn project_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_projects)
        .service(create_project)
        .service(delete_project)
        .service(get_project)
        .service(update_project)
        .service(check_user_admin_permission_for_project)
        .service(create_curl_group)
        .service(get_curl_group)
        .service(update_curl_group);
}
