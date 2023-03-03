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
        let public_project = app.get_test_public_project();

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
        let public_project = app.get_test_public_project();
        let private_project = app.get_test_private_project();

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
        let public_project = app.get_test_public_project();

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
        let public_project = app.get_test_public_project();
        let private_project = app.get_test_private_project();

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

        let public_project = app.get_test_public_project();
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

        let public_project = app.get_test_public_project();
        let response = app.delete_project(&public_project).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn deleting_project_sucessfully_returns_204() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let public_project = app.get_test_public_project();
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

        let mut fake_project = app.get_test_public_project();
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

        let project = app.get_test_private_project();
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

        let project = app.get_test_public_project();
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

        let project = app.get_test_public_project();

        let response = app.update_project(&project).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn updating_project_as_project_admin_returns_204() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let mut project = app.get_test_public_project();
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

        let mut project = app.get_test_public_project();
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

        let curl_group = app.get_test_curl_group();

        let response = app.create_curl_group(1, &curl_group).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn creating_group_while_not_logged_in_returns_401() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let project = app.get_test_public_project();
        app.create_project(&project).await;
        app.logout().await;

        let curl_group = app.get_test_curl_group();
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

        let project = app.get_test_public_project();
        app.create_project(&project).await;

        let curl_group = app.get_test_curl_group();
        let response = app.create_curl_group(1, &curl_group).await;
        assert_eq!(response.status(), StatusCode::OK);

        let response_id: Id = response.json().await.unwrap();
        assert_eq!(response_id.id, 1);
    }

    #[tokio::test]
    async fn creating_group_for_private_project_while_not_permitted_returns_403() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let project = app.get_test_private_project();
        app.create_project(&project).await;

        app.logout().await;
        app.signup("integration-other-user").await;

        let curl_group = app.get_test_curl_group();
        let response = app.create_curl_group(1, &curl_group).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}

#[cfg(test)]
mod get_curl_group {
    use backend::models::CurlGroup;

    use super::*;

    #[tokio::test]
    async fn getting_non_existent_group_returns_404() {
        let app = common::spawn_test_app().await;

        let response = app.get_curl_group(1).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        app.signup("integration-test").await;
        let response = app.get_curl_group(1).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn getting_curl_group_when_project_collaborator_returns_200() {
        // TODO
    }

    #[tokio::test]
    async fn getting_curl_group_when_project_admin_returns_200() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let project = app.get_test_public_project();
        let curl_group = app.get_test_curl_group();
        app.create_project(&project).await;
        app.create_curl_group(project.id, &curl_group).await;

        let response = app.get_curl_group(1).await;
        assert_eq!(response.status(), StatusCode::OK);
        let response_curl_group: CurlGroup = response.json().await.unwrap();
        assert_eq!(curl_group, response_curl_group);
    }

    #[tokio::test]
    async fn getting_private_curl_group_when_not_project_admin_or_collaborator_returns_403() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let mut project = app.get_test_private_project();
        project.id = 1;
        let curl_group = app.get_test_curl_group();
        app.create_project(&project).await;
        app.create_curl_group(1, &curl_group).await;

        app.logout().await;
        app.signup("integration-test-other-user").await;

        let response = app.get_curl_group(1).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        let response_curl_group: Result<CurlGroup, reqwest::Error> = response.json().await;
        assert!(response_curl_group.is_err());
    }

    #[tokio::test]
    async fn getting_private_curl_group_when_not_logged_in_returns_401() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let mut project = app.get_test_private_project();
        project.id = 1;
        let curl_group = app.get_test_curl_group();
        app.create_project(&project).await;
        app.create_curl_group(project.id, &curl_group).await;

        app.logout().await;

        let response = app.get_curl_group(1).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let response_curl_group: Result<CurlGroup, reqwest::Error> = response.json().await;
        assert!(response_curl_group.is_err());
    }

    #[tokio::test]
    async fn getting_public_curl_group_when_not_admin_or_collaborator_in_returns_200() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let project = app.get_test_public_project();
        let curl_group = app.get_test_curl_group();
        app.create_project(&project).await;
        app.create_curl_group(project.id, &curl_group).await;

        app.logout().await;
        app.signup("integration-test-other-user").await;

        let response = app.get_curl_group(1).await;
        assert_eq!(response.status(), StatusCode::OK);
        let response_curl_group: CurlGroup = response.json().await.unwrap();
        assert_eq!(curl_group, response_curl_group);
    }

    #[tokio::test]
    async fn getting_public_curl_group_when_not_logged_in_returns_200() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let project = app.get_test_public_project();
        let curl_group = app.get_test_curl_group();
        app.create_project(&project).await;
        app.create_curl_group(project.id, &curl_group).await;

        app.logout().await;

        let response = app.get_curl_group(1).await;
        assert_eq!(response.status(), StatusCode::OK);
        let response_curl_group: CurlGroup = response.json().await.unwrap();
        assert_eq!(curl_group, response_curl_group);
    }
}

#[cfg(test)]
mod update_curl_group {
    use backend::models::CurlGroup;

    use super::*;

    #[tokio::test]
    async fn updating_non_existent_group_returns_404() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let curl_group = app.get_test_curl_group();
        let response = app.update_curl_group(&curl_group).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn updating_curl_group_when_project_collaborator_returns_204() {
        // TODO
    }

    #[tokio::test]
    async fn updating_curl_group_when_project_admin_returns_204() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let mut project = app.get_test_private_project();
        project.id = 1;
        app.create_project(&project).await;

        let mut curl_group = app.get_test_curl_group();
        app.create_curl_group(project.id, &curl_group).await;

        curl_group.description = "I have been updated!".to_string();
        curl_group.curls = "updated.com".to_string();
        curl_group.labels = "1,2,3".to_string();
        curl_group.name = "Updated curl group".to_string();
        let response = app.update_curl_group(&curl_group).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let curl_group_from_db: CurlGroup = app
            .get_curl_group(curl_group.id)
            .await
            .json()
            .await
            .unwrap();
        assert_eq!(curl_group, curl_group_from_db);
    }

    #[tokio::test]
    async fn updating_private_project_curl_group_when_not_collaborator_or_admin_returns_403() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let mut project = app.get_test_private_project();
        project.id = 1;
        app.create_project(&project).await;

        let curl_group = app.get_test_curl_group();
        app.create_curl_group(project.id, &curl_group).await;

        app.logout().await;
        app.signup("integration-test-other-user").await;

        let response = app.update_curl_group(&curl_group).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn updating_public_project_curl_group_when_logged_in_returns_204() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let project = app.get_test_public_project();
        app.create_project(&project).await;

        let mut curl_group = app.get_test_curl_group();
        app.create_curl_group(project.id, &curl_group).await;

        app.logout().await;
        app.signup("integration-test-other-user").await;

        curl_group.description = "Updated".to_string();
        let response = app.update_curl_group(&curl_group).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn updating_project_when_not_logged_in_returns_401() {
        let app = common::spawn_test_app().await;
        app.signup("integration-test").await;

        let public_project = app.get_test_private_project();
        let private_project = app.get_test_private_project();
        app.create_project(&public_project).await;
        app.create_project(&private_project).await;

        let mut curl_group_public = app.get_test_curl_group();
        let mut curl_group_private = app.get_test_curl_group();
        curl_group_private.project_id = 2;

        app.create_curl_group(public_project.id, &curl_group_public)
            .await;
        app.create_curl_group(private_project.id, &curl_group_private)
            .await;

        app.logout().await;

        curl_group_public.description = "Updated".to_string();
        curl_group_private.description = "Updated".to_string();

        let response = app.update_curl_group(&curl_group_public).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let response = app.update_curl_group(&curl_group_private).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
