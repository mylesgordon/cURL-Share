use crate::Project;
use actix_web::{web, Responder, HttpResponse, get};
use sqlx::SqlitePool;

#[get("/project")]
async fn get_projects(pool: web::Data<SqlitePool>) -> impl Responder {
    let pool: &SqlitePool = &pool;
    let projects = sqlx::query_as!(Project, r#"SELECT * FROM project"#)
        .fetch_all(pool)
        .await;

    match projects {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub fn project_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_projects);
}