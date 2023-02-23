mod common;
use reqwest::StatusCode;

#[cfg(test)]
mod sign_up {
    use reqwest::Response;

    use crate::common::TestApplication;

    use super::*;

    async fn signup_and_assert(
        app: &TestApplication,
        asserted_status_code: StatusCode,
    ) -> Response {
        let response = app.signup().await;
        assert_eq!(response.status(), asserted_status_code);
        response
    }

    #[tokio::test]
    async fn successful_signup_returns_201_with_cookie() {
        let app = common::spawn_test_app().await;
        let response = signup_and_assert(&app, StatusCode::CREATED).await;
        assert!(response.cookies().count() == 1);
    }

    #[tokio::test]
    async fn duplicate_signup_returns_409() {
        let app = common::spawn_test_app().await;
        signup_and_assert(&app, StatusCode::CREATED).await;
        let response = signup_and_assert(&app, StatusCode::CONFLICT).await;
        assert!(response.cookies().count() == 0);
    }

    // TODO: sign up requirements
}

#[cfg(test)]
mod log_in {
    use super::*;

    #[tokio::test]
    async fn successful_login_returns_200_with_cookie() {
        let app = common::spawn_test_app().await;
        app.signup().await;

        let response = app.login_default_user().await;
        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.cookies().count() == 1);
    }

    #[tokio::test]
    async fn failed_login_returns_401() {
        let app = common::spawn_test_app().await;
        app.signup().await;

        let test_data = [
            ("integration", "test_wrong_password"),
            ("fake_user", "test"),
        ];

        for (username, password) in test_data {
            let response = app.login(username, password).await;
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
            assert!(response.cookies().count() == 0);
        }
    }
}

#[cfg(test)]
mod log_out {
    use super::*;

    #[tokio::test]
    async fn successful_logout_returns_204_with_purged_cookie() {
        let app = common::spawn_test_app().await;
        app.signup().await;

        let response = app.logout().await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        assert!(response.cookies().count() == 0);
    }
}
