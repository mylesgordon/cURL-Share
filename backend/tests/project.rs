mod common;
use crate::common::TestApplication;
use backend::models::Project;
use backend::routes::project::types::Id;
use reqwest::cookie::Cookie;
use reqwest::Response;
use reqwest::StatusCode;

// TODO: split helpers into its own file

#[cfg(test)]
mod create_project {
    use super::*;

    #[tokio::test]
    async fn creating_new_public_project_returns_200_and_is_visible() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;
        let public_project = app.get_public_project();

        let response = app.create_project(&public_project).await;
        assert_eq!(response.status(), StatusCode::OK);

        let response_id: Id = response.json().await.unwrap();
        assert_eq!(response_id.id, 1);

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
    async fn creating_new_private_project_returns_200_and_is_visible() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;
        let public_project = app.get_public_project();
        let private_project = app.get_private_project();

        app.create_project(&public_project).await;
        let response = app.create_project(&private_project).await;
        assert_eq!(response.status(), StatusCode::OK);

        let response_id: Id = response.json().await.unwrap();
        assert_eq!(response_id.id, 2);

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
        app.signup("integration-test").await;
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
        app.signup("integration-test").await;

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
        app.signup("integration-test").await;

        let public_project = app.get_public_project();
        let response = app.delete_project(&public_project).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn deleting_project_sucessfully_returns_204() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

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

    // Test 6 - Successful when public and not logged in + logged in

    #[tokio::test]
    async fn getting_non_existent_project_returns_404() {
        let app = common::spawn_test_app().await;

        let mut fake_project = app.get_public_project();
        fake_project.id = 5;

        let response = app.get_project(&fake_project, None).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn getting_private_project_as_admin_returns_200() {
        // TODO
    }

    #[tokio::test]
    async fn getting_private_project_as_collaborator_returns_200() {
        // TODO
    }

    #[tokio::test]
    async fn getting_private_project_as_non_logged_in_user_returns_401() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let project = app.get_private_project();
        app.create_project(&project).await;

        app.logout().await;
        let response = app.get_project(&project, Some(1)).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn getting_private_project_as_non_permitted_user_returns_403() {
        // TODO
    }

    #[tokio::test]
    async fn getting_public_project_as_non_logged_in_user_returns_200() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let project = app.get_public_project();
        app.create_project(&project).await;

        app.logout().await;
        let response = app.get_project(&project, None).await;
        assert_eq!(response.status(), StatusCode::OK);

        let project_from_db: Project = response.json().await.unwrap();
        assert_eq!(project_from_db, project);
    }
}

#[cfg(test)]
mod update_project {
    use super::*;

    #[tokio::test]
    async fn updating_non_existent_project_returns_404() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let project = app.get_public_project();

        let response = app.update_project(&project).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn updating_project_as_project_admin_returns_204() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let mut project = app.get_public_project();
        app.create_project(&project).await;

        project.description = "Woah it's been updated!".to_string();
        let response = app.update_project(&project).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let project_from_server: Project = app
            .get_project(&project, None)
            .await
            .json()
            .await
            .expect("Failed to serialise response.");
        assert_eq!(
            project_from_server.description,
            "Woah it's been updated!".to_string()
        );
    }

    #[tokio::test]
    async fn updating_project_not_as_project_admin_returns_403() {
        // TODO
    }

    #[tokio::test]
    async fn updating_project_not_logged_in_returns_401() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let mut project = app.get_public_project();
        app.create_project(&project).await;
        app.logout().await;

        project.description = "Updated again!".to_string();

        let response = app.update_project(&project).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let project_from_server: Project = app
            .get_project(&project, None)
            .await
            .json()
            .await
            .expect("Failed to serialise response.");
        assert_eq!(
            project_from_server.description,
            "A test public project".to_string()
        );
    }
}

#[cfg(test)]
mod create_curl_group {
    use super::*;

    #[tokio::test]
    async fn creating_group_for_non_existent_project_returns_404() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let curl_group = app.get_curl_group();

        let response = app.create_curl_group(1, &curl_group).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn creating_group_while_not_logged_in_returns_401() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let project = app.get_public_project();
        app.create_project(&project).await;
        app.logout().await;

        let curl_group = app.get_curl_group();
        let response = app.create_curl_group(1, &curl_group).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn creating_group_while_project_collaborator_returns_200() {
        // TODO
    }

    #[tokio::test]
    async fn creating_group_while_project_admin_returns_200() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let project = app.get_public_project();
        app.create_project(&project).await;

        let curl_group = app.get_curl_group();
        let response = app.create_curl_group(1, &curl_group).await;
        assert_eq!(response.status(), StatusCode::OK);

        let response_id: Id = response.json().await.unwrap();
        assert_eq!(response_id.id, 1);
    }

    #[tokio::test]
    async fn creating_group_for_private_project_while_not_permitted_returns_403() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let project = app.get_private_project();
        app.create_project(&project).await;

        app.logout().await;
        app.signup("integration-other-user").await;

        let curl_group = app.get_curl_group();
        let response = app.create_curl_group(1, &curl_group).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
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
