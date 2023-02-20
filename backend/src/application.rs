use std::net::TcpListener;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, dev::Server, web, App, HttpServer};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tracing_actix_web::TracingLogger;

use crate::routes::{health_check, project_routes, user_routes};

pub enum ApplicationPoolSettings {
    Standard,
    Test,
}

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(
        port: u16,
        pool_settings: ApplicationPoolSettings,
    ) -> Result<Self, std::io::Error> {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&address)?;

        let db_pool = match pool_settings {
            ApplicationPoolSettings::Standard => {
                SqlitePool::connect("sqlite:data.db").await.unwrap()
            }
            ApplicationPoolSettings::Test => Self::get_sqlite_test_pool().await,
        };

        let port = listener.local_addr()?.port();
        let server = run(db_pool, listener)?;
        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    async fn get_sqlite_test_pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .idle_timeout(None)
            .max_lifetime(None)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to migrate database");

        pool
    }
}

fn run(db_pool: SqlitePool, listener: TcpListener) -> Result<Server, std::io::Error> {
    tracing::info!("Starting server at {}", listener.local_addr()?);

    let private_key = Key::generate();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), private_key.clone())
                    .build(),
            )
            .app_data(web::Data::new(db_pool.clone()))
            .service(
                web::scope("/api/v1")
                    .route("/health-check", web::get().to(health_check))
                    .configure(project_routes)
                    .configure(user_routes),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
