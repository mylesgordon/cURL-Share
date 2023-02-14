use crate::Project;
use actix_web::{web, Responder, HttpResponse, get};
use sqlx::SqlitePool;

#[get("/project")]
async fn get_projects(pool: web::Data<SqlitePool>) -> impl Responder {
    // let pool: &SqlitePool = &pool;
    // let maybe_todos = sqlx::query_as!(Project, r#"SELECT * FROM project"#)
    //     .fetch_all(pool)
    //     .await;

    // match maybe_todos {
    //     Ok(todos) => HttpResponse::Ok().json(todos),
    //     Err(_) => HttpResponse::InternalServerError().into(),
    // }
    "Project!"
}

pub fn project_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_projects);
}