use backend::{
    application::{Application, ApplicationPoolSettings},
    observability::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;

static LOGS: Lazy<()> = Lazy::new(|| {
    let subscriber_name = "integration";
    let default_filter_level = "info";

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApplication {
    port: u16,
}

impl TestApplication {
    fn get_test_user(&self) -> serde_json::Value {
        serde_json::json!({"username": "integration", "password": "test"})
    }

    fn generate_url(&self, suffix: &'static str) -> String {
        format!("http://localhost:{}/api/v1/{}", self.port, suffix)
    }

    pub async fn health_check(&self) -> reqwest::Response {
        let url = self.generate_url("health-check");
        reqwest::Client::new()
            .get(url)
            .send()
            .await
            .expect("Failed to send health check request")
    }

    pub async fn login_default_user(&self) -> reqwest::Response {
        self.login("integration", "test").await
    }

    pub async fn login(&self, username: &'static str, password: &'static str) -> reqwest::Response {
        let url = self.generate_url("log-in");
        let data = serde_json::json!({"username": username, "password": password});

        reqwest::Client::new()
            .post(url)
            .json(&data)
            .send()
            .await
            .expect("Failed to send sign in request")
    }

    pub async fn logout(&self) -> reqwest::Response {
        let url = self.generate_url("log-out");

        reqwest::Client::new()
            .post(url)
            .send()
            .await
            .expect("Failed to send log out request")
    }

    pub async fn signup(&self) -> reqwest::Response {
        let url = self.generate_url("sign-up");
        let data = self.get_test_user();

        reqwest::Client::new()
            .post(url)
            .json(&data)
            .send()
            .await
            .expect("Failed to send sign up request")
    }
}

pub async fn spawn_test_app() -> TestApplication {
    Lazy::force(&LOGS);

    let application = Application::build(0, ApplicationPoolSettings::Test)
        .await
        .expect("Failed to build test application");
    let port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    TestApplication { port }
}
