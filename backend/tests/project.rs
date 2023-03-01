mod common;
use crate::common::TestApplication;
use backend::models::Project;
use reqwest::cookie::Cookie;
use reqwest::Response;
use reqwest::StatusCode;

// TODO: split helpers into its own file

#[cfg(test)]
mod create_project {
    use super::*;

    #[tokio::test]
    async fn creating_new_public_project_returns_204_and_is_visible() {
        let app = common::spawn_test_app().await;
        app.signup().await;
        let public_project = app.get_public_project();

        let response = app.create_project(&public_project).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let projects: Vec<Project> = app
            .get_projects()
            .await
            .json()
            .await
            .expect("Failed to serialise response");
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0], public_project);
    }

    #[tokio::test]
    async fn creating_new_private_project_returns_204_and_is_visible() {
        let app = common::spawn_test_app().await;
        app.signup().await;
        let public_project = app.get_public_project();
        let private_project = app.get_private_project();

        app.create_project(&public_project).await;
        let response = app.create_project(&private_project).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let projects: Vec<Project> = app
            .get_projects()
            .await
            .json()
            .await
            .expect("Failed to serialise response");
        assert_eq!(projects.len(), 2);
        assert_eq!(projects[0], public_project);
    }

    #[tokio::test]
    async fn attempting_to_create_new_project_not_logged_in_returns_401() {
        let app = common::spawn_test_app().await;
        let public_project = app.get_public_project();

        let response = app.create_project(&public_project).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}

#[cfg(test)]
mod get_projects {
    use backend::models::Project;

    use super::*;

    #[tokio::test]
    async fn get_projects_depends_on_whether_user_is_logged_in() {
        let app = common::spawn_test_app().await;
        app.signup().await;
        let public_project = app.get_public_project();
        let private_project = app.get_private_project();

        app.create_project(&public_project).await;
        app.create_project(&private_project).await;

        let projects: Vec<Project> = app
            .get_projects()
            .await
            .json()
            .await
            .expect("Failed to serialise response");
        assert_eq!(projects.len(), 2);

        app.logout().await;

        let projects: Vec<Project> = app
            .get_projects()
            .await
            .json()
            .await
            .expect("Failed to serialise response");
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0], public_project);
    }
}

#[cfg(test)]
mod delete_project {
    use super::*;

    async fn assert_number_of_projects(app: &TestApplication, number: usize) {
        let projects: Vec<Project> = app
            .get_projects()
            .await
            .json()
            .await
            .expect("Failed to serialise response");
        assert_eq!(projects.len(), number);
    }

    // TODO: test cascades once collaborators/admins implemented

    #[tokio::test]
    async fn deleting_project_not_logged_in_returns_401() {
        let app = common::spawn_test_app().await;
        app.signup().await;

        let public_project = app.get_public_project();
        app.create_project(&public_project).await;
        app.logout().await;

        let response = app.delete_project(&public_project).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_number_of_projects(&app, 1).await;
    }

    #[tokio::test]
    async fn deleting_project_not_as_admin_returns_403() {
        // TODO
    }

    #[tokio::test]
    async fn deleting_non_existent_project_returns_404() {
        let app = common::spawn_test_app().await;
        app.signup().await;

        let public_project = app.get_public_project();
        let response = app.delete_project(&public_project).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn deleting_project_sucessfully_returns_204() {
        let app = common::spawn_test_app().await;
        app.signup().await;

        let public_project = app.get_public_project();
        app.create_project(&public_project).await;

        let response = app.delete_project(&public_project).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        assert_number_of_projects(&app, 0).await;
    }
}

#[cfg(test)]
mod get_project {
    use super::*;

    // Test 1 - 404 for non existent project
    // Test 2 - Succesful when project admin and private
    // Test 3 - Succesful when project collaborator and private
    // Test 4 - Fails when private and not collaborator/admin
    // Test 5 - Fails when not logged in and private
    // Test 6 - Successful when public and not logged in + logged in
}

#[cfg(test)]
mod update_project {
    use super::*;

    // Test 1 - 404 for non existent project
    // Test 2 - Successful when user is project admin
    // Test 3 - Fails when user is not project admin
    // Test 4 - Fails when user is not logged in
}

#[cfg(test)]
mod create_curl_group {
    use super::*;

    // Test 1 - 404 for non existent project
    // Test 2 - Succesful when project collaborator
    // Test 3 - Succesful when project admin
    // Test 4 - Fails when private and not collaborator/admin
    // Test 5 - Succesful when logged in and public
    // Test 6 - Fails when not logged in
}

#[cfg(test)]
mod get_curl_group {
    use super::*;

    // test 1 - 404 for non existent group
    // test 2 - succesful when project collaborator
    // test 3 - succesful when project admin
    // test 4 - fails when private and not collaborator/admin
    // test 5 - successful for not logged in/logged in and public
}

#[cfg(test)]
mod update_curl_group {
    use super::*;

    // test 1 - 404 for non existent group
    // test 2 - succesful when project collaborator
    // test 3 - succesful when project admin
    // test 4 - fails when private and not collaborator/admin
    // test 5 - successful when logged in and public
    // test 6 - fails when not logged in
}
