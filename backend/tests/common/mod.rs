use backend::{
    application::{Application, ApplicationPoolSettings},
    models::{CurlGroup, Project, ProjectInfo},
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
    url: String,
}

impl TestApplication {
    // helpers
    fn generate_url(&self, suffix: String) -> String {
        format!("{}/api/v1/{}", self.url, suffix)
    }

    pub async fn add_other_user_as_collaborator_and_login(&self, project: &mut Project) {
        self.logout().await;
        self.signup("integration-test-other-user").await;

        self.logout().await;
        self.login("integration-test", "test").await;

        project
            .collaborators
            .push("integration-test-other-user".to_string());
        self.update_project(&project).await;

        self.logout().await;
        self.login("integration-test-other-user", "test").await;
    }

    pub async fn health_check(&self) -> reqwest::Response {
        let url = self.generate_url("health-check".to_string());
        self.client
            .get(url)
            .send()
            .await
            .expect("Failed to send health check request")
    }

    pub fn get_test_curl_group(&self) -> CurlGroup {
        CurlGroup {
            id: 1,
            curls: "some-curl".to_string(),
            description: "This is a test curl group".to_string(),
            labels: "test1,test2".to_string(),
            name: "CurlGroup 1".to_string(),
            project_id: 1,
        }
    }

    pub fn get_test_public_project(&self) -> Project {
        Project {
            admins: vec!["integration-test".to_string()],
            collaborators: vec![],
            info: ProjectInfo {
                id: 1,
                name: "Public Project".to_string(),
                environments: "localhost:8080,https://coolurl.com".to_string(),
                description: "A test public project".to_string(),
                visibility: "Public".to_string(),
            },
            groups: vec![],
        }
    }

    pub fn get_test_private_project(&self) -> Project {
        Project {
            admins: vec!["integration-test".to_string()],
            collaborators: vec![],
            info: ProjectInfo {
                id: 2,
                name: "Private Project".to_string(),
                environments: "localhost:1800,https://coolurl.co.uk".to_string(),
                description: "A test private project".to_string(),
                visibility: "Private".to_string(),
            },
            groups: vec![],
        }
    }

    // project
    pub async fn create_project(&self, project: &ProjectInfo) -> reqwest::Response {
        let url = self.generate_url("project".to_string());
        self.client
            .post(url)
            .json(&project)
            .send()
            .await
            .expect("Failed to send create project request")
    }

    pub async fn delete_project(&self, project: &ProjectInfo) -> reqwest::Response {
        let url = self.generate_url(format!("project/{}", project.id));
        self.client
            .delete(url)
            .send()
            .await
            .expect("Failed to send create project request")
    }

    pub async fn get_project(
        &self,
        project: &Project,
        id_override: Option<i64>,
    ) -> reqwest::Response {
        let project_id = match id_override {
            Some(id) => id,
            None => project.info.id,
        };
        let url = self.generate_url(format!("project/{}", project_id));

        self.client
            .get(url)
            .send()
            .await
            .expect("Failed to send get project request")
    }

    pub async fn get_projects(&self) -> reqwest::Response {
        let url = self.generate_url("project".to_string());
        self.client
            .get(url)
            .send()
            .await
            .expect("Failed to send get project request")
    }

    pub async fn update_project(&self, project: &Project) -> reqwest::Response {
        let url = self.generate_url(format!("project/{}", project.info.id));

        self.client
            .post(url)
            .json(project)
            .send()
            .await
            .expect("Failed to send update project request")
    }

    pub async fn is_user_admin_of_project(&self, project: &Project) -> reqwest::Response {
        let url = self.generate_url(format!("project/{}/is-user-admin", project.info.id));

        self.client
            .get(url)
            .send()
            .await
            .expect("Failed to send update project request")
    }

    pub async fn create_curl_group(
        &self,
        project_id: i64,
        curl_group: &CurlGroup,
    ) -> reqwest::Response {
        let url = self.generate_url(format!("project/{}/group", project_id));

        self.client
            .post(url)
            .json(curl_group)
            .send()
            .await
            .expect("Failed to send create curl group request")
    }

    pub async fn get_curl_group(&self, id: i64) -> reqwest::Response {
        let url = self.generate_url(format!("group/{}", id));

        self.client
            .get(url)
            .send()
            .await
            .expect("Failed to send get curl group request")
    }

    pub async fn update_curl_group(&self, curl_group: &CurlGroup) -> reqwest::Response {
        let url = self.generate_url(format!("group/{}", curl_group.id));

        self.client
            .post(url)
            .json(curl_group)
            .send()
            .await
            .expect("Failed to send update curl group request")
    }

    // user
    pub async fn delete_user(&self) -> reqwest::Response {
        let url = self.generate_url("delete-user".to_string());
        self.client
            .post(url)
            .send()
            .await
            .expect("Failed to send delete user request")
    }

    pub async fn login(&self, username: &'static str, password: &'static str) -> reqwest::Response {
        let url = self.generate_url("log-in".to_string());
        let data = serde_json::json!({"username": username, "password": password});

        self.client
            .post(url)
            .json(&data)
            .send()
            .await
            .expect("Failed to send sign in request")
    }

    pub async fn logout(&self) -> reqwest::Response {
        let url = self.generate_url("log-out".to_string());

        self.client
            .post(url)
            .send()
            .await
            .expect("Failed to send log out request")
    }

    pub async fn signup(&self, username: &'static str) -> reqwest::Response {
        let url = self.generate_url("sign-up".to_string());
        let data = serde_json::json!({"username": username.to_string(), "password": "test"});

        self.client
            .post(url)
            .json(&data)
            .send()
            .await
            .expect("Failed to send sign up request")
    }

    pub async fn user_status(&self) -> reqwest::Response {
        let url = self.generate_url("user-status".to_string());

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
    let url = format!("http://localhost:{}", application.port());

    let _ = tokio::spawn(application.run_until_stopped());

    TestApplication { client, url }
}
