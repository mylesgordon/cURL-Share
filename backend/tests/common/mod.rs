use backend::application::Application;
use sqlx::{SqlitePool};

pub struct TestApplication {
    port: u16
}

impl TestApplication {
    fn generate_url(&self, suffix: &'static str) -> String {
        format!("http://localhost:{}/api/v1/{}", self.port, suffix)
    }

    pub async fn health_check(&self) -> reqwest::Response {
        let url = self.generate_url("health-check");
        reqwest::Client::new().get(url).send().await.expect("Failed to send health check request")
    }
}

pub async fn spawn_test_app() -> TestApplication {
    let test_db_pool = SqlitePool::connect("sqlite::memory:").await.expect("Failed to open sqlite memory database");
    sqlx::migrate!("./migrations").run(&test_db_pool).await.expect("Failed to migrate database");

    let application = Application::build("sqlite::memory:", 0).await.expect("Failed to build test application");
    let port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    TestApplication {port}
}
