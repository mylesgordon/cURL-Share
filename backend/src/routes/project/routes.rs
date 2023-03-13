use crate::{
    helpers::get_user_id,
    models::{CurlGroup, Project, ProjectInfo},
    routes::project::types::Id,
};

use actix_session::Session;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use sqlx::SqlitePool;

use super::types::{ProjectError, UserAdminStatus};

#[get("/project")]
#[tracing::instrument(name = "Getting projects.", skip(pool, session))]
async fn get_projects(
    // query: web::Query<ProjectParams>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    // TODO: handle search query & labels
    match get_projects_from_db(&pool, &session).await {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(e) => e.into(),
    }
}

#[post("/project")]
#[tracing::instrument(name = "Creating new project.", skip(body, pool, session))]
async fn create_project(
    body: web::Json<ProjectInfo>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    match insert_project_into_db(&body, &pool, &session).await {
        Ok(id) => HttpResponse::Ok().json(Id { id }),
        Err(e) => e.into(),
    }
}

#[delete("/project/{project_id}")]
#[tracing::instrument(name = "Deleting project.", skip(pool, session))]
async fn delete_project(
    params: web::Path<i64>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    match delete_project_from_db(*params, &pool, &session).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.into(),
    }
}

#[get("/project/{project_id}")]
#[tracing::instrument(name = "Getting project.", skip(pool, session))]
async fn get_project(
    params: web::Path<i64>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    match get_project_from_db(*params, &pool, &session).await {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(e) => e.into(),
    }
}

#[post("/project/{project_id}")]
#[tracing::instrument(name = "Updating project.", skip(body, pool, session))]
async fn update_project(
    params: web::Path<i64>,
    body: web::Json<Project>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    match update_project_in_db(*params, &body, &pool, &session).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.into(),
    }
}

#[post("/project/{project_id}/group")]
#[tracing::instrument(name = "Creating curl group.", skip(body, pool, session))]
async fn create_curl_group(
    params: web::Path<i64>,
    body: web::Json<CurlGroup>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    match insert_curl_group_into_db(*params, &body, &pool, &session).await {
        Ok(curl_group_id) => HttpResponse::Ok().json(Id { id: curl_group_id }),
        Err(e) => e.into(),
    }
}

#[get("/project/{project_id}/is-user-admin")]
#[tracing::instrument(name = "Getting user admin status.", skip(pool, session))]
async fn check_user_admin_permission_for_project(
    params: web::Path<i64>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    let is_user_admin = match get_user_id(&session).await {
        Ok(user_id) => check_user_has_project_admin_permission(user_id, *params, &pool)
            .await
            .is_ok(),
        Err(_) => false,
    };
    HttpResponse::Ok().json(UserAdminStatus { is_user_admin })
}

#[get("/group/{group_id}")]
#[tracing::instrument(name = "Getting curl group.", skip(pool, session))]
async fn get_curl_group(
    params: web::Path<i64>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    match get_curl_group_from_db(*params, &pool, &session).await {
        Ok(curl_group) => HttpResponse::Ok().json(curl_group),
        Err(e) => e.into(),
    }
}

#[post("/group/{group_id}")]
#[tracing::instrument(name = "Updating curl group.", skip(body, pool, session))]
async fn update_curl_group(
    params: web::Path<i64>,
    body: web::Json<CurlGroup>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    match update_curl_group_in_db(*params, &body, &pool, &session).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.into(),
    }
}

async fn insert_curl_group_into_db(
    project_id: i64,
    curl_group: &CurlGroup,
    pool: &SqlitePool,
    session: &Session,
) -> Result<i64, ProjectError> {
    let user_id = get_user_id(session).await?;
    check_user_has_project_permission(user_id, project_id, pool).await?;

    let curl_group_id = sqlx::query!(r#"INSERT INTO curl_group (curls, description, labels, name, project_id) VALUES (?, ?, ?, ?, ?) RETURNING id"#, 
        curl_group.curls, curl_group.description, curl_group.labels, curl_group.name, project_id)
        .fetch_one(pool)
        .await?.id;

    Ok(curl_group_id)
}

async fn update_curl_group_in_db(
    curl_group_id: i64,
    curl_group: &CurlGroup,
    pool: &SqlitePool,
    session: &Session,
) -> Result<(), ProjectError> {
    get_user_id(session).await?;
    curl_group_check_user_permission(curl_group_id, pool, session).await?;

    sqlx::query!(
        r#"UPDATE curl_group SET curls = ?, description = ?, labels = ?, name = ? WHERE id = ?"#,
        curl_group.curls,
        curl_group.description,
        curl_group.labels,
        curl_group.name,
        curl_group_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn check_user_has_project_permission(
    user_id: i64,
    project_id: i64,
    pool: &SqlitePool,
) -> Result<(), ProjectError> {
    let admin_query = check_user_has_project_admin_permission(user_id, project_id, pool).await;
    let collaborator_query = sqlx::query!(
        r#"SELECT * FROM project_collaborator WHERE project_id = ? AND user_id = ?"#,
        project_id,
        user_id
    )
    .fetch_one(pool)
    .await;

    if admin_query.is_ok() || collaborator_query.is_ok() {
        Ok(())
    } else {
        // Return 404 if project not found - else just return forbidden
        let mut error = ProjectError::Forbidden(
            "User does not not have permission to perform that action on this project.".to_string(),
        );

        if let ProjectError::ProjectDoesNotExistError(e) = admin_query.unwrap_err() {
            error = ProjectError::ProjectDoesNotExistError(e)
        }

        Err(error)
    }
}

async fn check_user_has_project_admin_permission(
    user_id: i64,
    project_id: i64,
    pool: &SqlitePool,
) -> Result<(), ProjectError> {
    let project_does_exist_query =
        sqlx::query!(r#"SELECT id FROM project WHERE id = ?"#, project_id)
            .fetch_one(pool)
            .await;

    if project_does_exist_query.is_err() {
        return Err(ProjectError::ProjectDoesNotExistError(
            "Requested project does not exist.".to_string(),
        ));
    }

    let project_admin_query = sqlx::query!(
        r#"SELECT * FROM project_admin WHERE project_id = ? AND user_id = ?"#,
        project_id,
        user_id
    )
    .fetch_one(pool)
    .await;

    if project_admin_query.is_err() {
        return Err(ProjectError::Forbidden(
            "User does not have permission to for this project".to_string(),
        ));
    }

    Ok(())
}

async fn curl_group_check_user_permission(
    group_id: i64,
    pool: &SqlitePool,
    session: &Session,
) -> Result<(), ProjectError> {
    let project_id = sqlx::query!(
        r#"SELECT project_id FROM curl_group WHERE id = ?"#,
        group_id
    )
    .fetch_one(pool)
    .await?
    .project_id;

    // checks project visibility and such things - we just discard the resulting project output
    get_project_from_db(project_id, pool, session).await?;

    Ok(())
}

async fn get_curl_group_from_db(
    group_id: i64,
    pool: &SqlitePool,
    session: &Session,
) -> Result<CurlGroup, ProjectError> {
    curl_group_check_user_permission(group_id, pool, session).await?;

    let curl_group = sqlx::query_as!(
        CurlGroup,
        r#"SELECT * FROM curl_group WHERE id = ?"#,
        group_id
    )
    .fetch_one(pool)
    .await?;

    Ok(curl_group)
}

async fn delete_project_from_db(
    project_id: i64,
    pool: &SqlitePool,
    session: &Session,
) -> Result<(), ProjectError> {
    let user_id = get_user_id(session).await?;
    check_user_has_project_admin_permission(user_id, project_id, pool).await?;

    sqlx::query!(r#"DELETE FROM project WHERE id = ?"#, project_id)
        .execute(pool)
        .await?;

    Ok(())
}

async fn get_project_from_db(
    project_id: i64,
    pool: &SqlitePool,
    session: &Session,
) -> Result<Project, ProjectError> {
    let info = sqlx::query_as!(
        ProjectInfo,
        r#"SELECT * FROM project WHERE id = ?"#,
        project_id
    )
    .fetch_one(pool)
    .await?;

    if info.visibility == "Private" {
        let user_id = get_user_id(session).await?;
        check_user_has_project_permission(user_id, project_id, pool).await?;
    }

    let (admins, collaborators) = get_admins_and_collaborators(info.id, pool).await?;
    let groups = get_curl_groups_for_project(project_id, pool).await?;
    let project = Project {
        info,
        admins,
        collaborators,
        groups,
    };

    Ok(project)
}

async fn get_projects_from_db(
    pool: &SqlitePool,
    session: &Session,
) -> Result<Vec<ProjectInfo>, ProjectError> {
    let user_id = get_user_id(session).await;

    let project_infos = match user_id {
        Ok(user_id) => {
            sqlx::query_as!(ProjectInfo, r#"
            SELECT id, environments, description, name, visibility FROM project
            LEFT JOIN project_admin ON project_admin.project_id = project.id AND project_admin.user_id = ?
            LEFT JOIN project_collaborator ON project_collaborator.project_id = project.id AND project_collaborator.user_id = ?
            WHERE visibility = "Public" OR project_admin.user_id = ? OR project_collaborator.user_id = ?;"#, user_id, user_id, user_id, user_id)
            .fetch_all(pool).await?
        },
        Err(_) => {
            sqlx::query_as!(
                ProjectInfo,
                r#"SELECT * FROM project WHERE visibility = "Public""#
            )
            .fetch_all(pool)
            .await?
        }
    };

    Ok(project_infos)
}

async fn insert_project_into_db(
    project_info: &ProjectInfo,
    pool: &SqlitePool,
    session: &Session,
) -> Result<i64, ProjectError> {
    let user_id = get_user_id(session).await?;

    let project_id = sqlx::query!(r#"INSERT INTO project (environments, description, name, visibility) VALUES (?, ?, ?, ?) RETURNING id"#, 
        project_info.environments, project_info.description, project_info.name, project_info.visibility)
        .fetch_one(pool)
        .await?.id;

    sqlx::query!(
        r#"INSERT INTO project_admin (project_id, user_id) VALUES (?, ?)"#,
        project_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(project_id)
}

async fn update_project_in_db(
    project_id: i64,
    project: &Project,
    pool: &SqlitePool,
    session: &Session,
) -> Result<(), ProjectError> {
    let user_id = get_user_id(session).await?;
    check_user_has_project_admin_permission(user_id, project_id, pool).await?;

    sqlx::query!(
        r#"UPDATE project SET environments = ?, description = ?, name = ?, visibility = ? WHERE id = ?"#,
        project.info.environments,
        project.info.description,
        project.info.name,
        project.info.visibility,
        project_id
    )
    .execute(pool)
    .await?;

    update_project_admins_and_collaborators(project, pool).await?;

    Ok(())
}

async fn update_project_admins_and_collaborators(
    project: &Project,
    pool: &SqlitePool,
) -> Result<(), ProjectError> {
    // TODO: use query builder eventually!

    let admin_list = generate_user_list(&project.admins).await;
    let collaborator_list = generate_user_list(&project.admins).await;
    sqlx::query!(
        r#"DELETE FROM project_admin WHERE project_id = ? AND user_id in (
        SELECT user_id FROM project_admin
        LEFT JOIN user ON user.id = project_admin.user_id
        WHERE user.name NOT IN (?)
    )"#,
        project.info.id,
        admin_list
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        r#"DELETE FROM project_collaborator WHERE project_id = ? AND user_id in (
        SELECT user_id FROM project_collaborator
        LEFT JOIN user ON user.id = project_collaborator.user_id
        WHERE user.name NOT IN (?)
    )"#,
        project.info.id,
        collaborator_list
    )
    .execute(pool)
    .await?;

    for admin in &project.admins {
        sqlx::query!(
            r#"INSERT INTO project_admin (project_id, user_id) VALUES (?, (SELECT id FROM user WHERE name = ?))"#,
            project.info.id, admin,
        ).execute(pool).await?;
    }

    for collaborator in &project.collaborators {
        sqlx::query!(
            r#"INSERT INTO project_collaborator (project_id, user_id) VALUES (?, (SELECT id FROM user WHERE name = ?))"#,
            project.info.id, collaborator,
        ).execute(pool).await?;
    }

    Ok(())
}

async fn get_admins_and_collaborators(
    project_id: i64,
    pool: &SqlitePool,
) -> Result<(Vec<String>, Vec<String>), ProjectError> {
    // TODO: tidy up
    let admins: Vec<String> = sqlx::query!(
        r#"SELECT name FROM user 
        LEFT JOIN project_admin ON project_admin.user_id = user.id
        WHERE project_admin.project_id = ?
    "#,
        project_id
    )
    .fetch_all(pool)
    .await?
    .iter()
    .map(|record| record.name.clone())
    .collect();

    let collaborators: Vec<String> = sqlx::query!(
        r#"SELECT name FROM user 
        LEFT JOIN project_collaborator ON project_collaborator.user_id = user.id
        WHERE project_collaborator.project_id = ?
    "#,
        project_id
    )
    .fetch_all(pool)
    .await?
    .iter()
    .map(|record| record.name.clone())
    .collect();

    Ok((admins, collaborators))
}

async fn get_curl_groups_for_project(
    project_id: i64,
    pool: &SqlitePool,
) -> Result<Vec<CurlGroup>, ProjectError> {
    let curl_groups: Vec<CurlGroup> = sqlx::query_as!(
        CurlGroup,
        r#"SELECT * FROM curl_group WHERE project_id = ?"#,
        project_id
    )
    .fetch_all(pool)
    .await?;

    Ok(curl_groups)
}

async fn generate_user_list(users: &Vec<String>) -> String {
    let mut users = users.clone();
    for i in 0..users.len() {
        users[i] = format!("\"{}\"", users[i]);
    }
    users.join(", ")
}

#[cfg(test)]
mod user_list_test {
    use super::generate_user_list;

    #[tokio::test]
    async fn generates_user_list_as_expected() {
        let users = vec!["user".to_string(), "other_user".to_string()];
        let user_list = generate_user_list(&users).await;
        assert_eq!(user_list, "\"user\", \"other_user\"");
    }

    #[tokio::test]
    async fn generates_blank_string_as_expected_when_no_users() {
        let users = vec![];
        let user_list = generate_user_list(&users).await;
        assert_eq!(user_list, "");
    }
}
