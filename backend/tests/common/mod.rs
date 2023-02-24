use backend::{
    application::{Application, ApplicationPoolSettings},
    observability::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;
use reqwest::{
    cookie::{Cookie, Jar},
    Client, Url,
};

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
    client: Client,
    port: u16,
    url: String,
}

impl TestApplication {
    fn get_test_user(&self) -> serde_json::Value {
        serde_json::json!({"username": "integration", "password": "test"})
    }

    fn generate_url(&self, suffix: &'static str) -> String {
        format!("{}/api/v1/{}", self.url, suffix)
    }

    pub async fn health_check(&self) -> reqwest::Response {
        let url = self.generate_url("health-check");
        self.client
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

        self.client
            .post(url)
            .json(&data)
            .send()
            .await
            .expect("Failed to send sign in request")
    }

    pub async fn logout(&self) -> reqwest::Response {
        let url = self.generate_url("log-out");

        self.client
            .post(url)
            .send()
            .await
            .expect("Failed to send log out request")
    }

    pub async fn signup(&self) -> reqwest::Response {
        let url = self.generate_url("sign-up");
        let data = self.get_test_user();

        self.client
            .post(url)
            .json(&data)
            .send()
            .await
            .expect("Failed to send sign up request")
    }

    pub async fn user_status(&self) -> reqwest::Response {
        let url = self.generate_url("user-status");

        self.client
            .get(url)
            .send()
            .await
            .expect("Failed to send user status request")
    }
}

pub async fn spawn_test_app() -> TestApplication {
    Lazy::force(&LOGS);

    let application = Application::build(0, ApplicationPoolSettings::Test)
        .await
        .expect("Failed to build test application");

    let client = Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to construct reqwest client");
    let port = application.port();
    let url = format!("http://localhost:{}", port);

    let _ = tokio::spawn(application.run_until_stopped());

    TestApplication { client, port, url }
}
