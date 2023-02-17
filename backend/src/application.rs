use std::net::TcpListener;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, dev::Server, middleware::Logger, web, App, HttpServer};
use log::info;
use sqlx::SqlitePool;

use crate::{
    routes::{project_routes, user_routes},
    status,
};

pub struct Application {
    server: Server,
}

impl Application {
    pub async fn build(port: u16) -> Result<Self, std::io::Error> {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&address)?;
        let db_pool = SqlitePool::connect("data.db").await.unwrap();

        let server = run(db_pool, listener)?;
        Ok(Self { server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

fn run(db_pool: SqlitePool, listener: TcpListener) -> Result<Server, std::io::Error> {
    info!("Starting server at {}", listener.local_addr()?);

    let private_key = Key::generate();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), private_key.clone())
                    .build(),
            )
            .app_data(web::Data::new(db_pool.clone()))
            .service(
                web::scope("/api/v1")
                    .route("/", web::get().to(status))
                    .configure(project_routes)
                    .configure(user_routes),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
