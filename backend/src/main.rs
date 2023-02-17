mod application;
mod models;
mod routes;

use crate::models::*;
use actix_web::{HttpResponse, Responder};
use application::Application;
use dotenv::dotenv;
use env_logger::Env;

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Test".to_string(),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let application = Application::build(8080).await?;
    application.run_until_stopped().await?;

    Ok(())
}
