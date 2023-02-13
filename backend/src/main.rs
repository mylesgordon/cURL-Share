mod models;

use crate::models::*;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;

async fn get_projects(pool: web::Data<SqlitePool>) -> impl Responder {
    let pool: &SqlitePool = &pool;
    let maybe_todos = sqlx::query_as!(Project, r#"SELECT * FROM project"#)
        .fetch_all(pool)
        .await;

    match maybe_todos {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Test".to_string(),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let bind_address = "127.0.0.1:8080";
    println!("Starting server at {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(status))
            .route("/project", web::get().to(get_projects))
    })
    .bind(bind_address)?
    .run()
    .await?;

    Ok(())
}

