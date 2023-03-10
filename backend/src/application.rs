use std::{net::TcpListener, env};

use actix_cors::Cors;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time, Key},
    dev::Server,
    web, App, HttpServer,
};
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
        let base_url = env::var("BASE_URL").unwrap_or_else(|_| "localhost".to_string());
        let address = format!("{}:{}", base_url, port);
        let listener = TcpListener::bind(&address)?;

        let db_pool = match pool_settings {
            ApplicationPoolSettings::Standard => {
                SqlitePool::connect("sqlite:data.db").await.unwrap()
            }
            ApplicationPoolSettings::Test => Self::get_sqlite_test_pool().await,
        };

        sqlx::migrate!("./migrations")
            .run(&db_pool)
            .await
            .expect("Failed to migrate database");

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

        pool
    }
}

fn run(db_pool: SqlitePool, listener: TcpListener) -> Result<Server, std::io::Error> {
    tracing::info!("Starting server at {}", listener.local_addr()?);

    let private_key = Key::generate();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(TracingLogger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), private_key.clone())
                    .cookie_same_site(actix_web::cookie::SameSite::Lax)
                    .cookie_secure(false)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(time::Duration::days(5)),
                    )
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
