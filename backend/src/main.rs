mod models;
mod routes;

use crate::{models::*, routes::*};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use sqlx::sqlite::SqlitePool;
use std::env;

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Test".to_string(),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let private_key = actix_web::cookie::Key::generate();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let bind_address = "127.0.0.1:8080";
    info!("Starting server at {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    private_key.clone(),
                )
                .build(),
            )
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/v1")
                    .route("/", web::get().to(status))
                    .configure(project_routes)
                    .configure(user_routes),
            )
    })
    .bind(bind_address)?
    .run()
    .await?;

    Ok(())
}
