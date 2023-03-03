use crate::{
    helpers::get_user_id,
    models::{CurlGroup, Project},
    routes::project::types::{Id, ProjectParams},
};

use actix_session::Session;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use sqlx::SqlitePool;

use super::types::ProjectError;

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
    body: web::Json<Project>,
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

        if admin_query.is_err() {
            match admin_query.unwrap_err() {
                ProjectError::ProjectDoesNotExistError(e) => {
                    error = ProjectError::ProjectDoesNotExistError(e)
                }
                _ => (),
            }
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
    curl_group_check_user_permission(group_id, &pool, &session).await?;

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
    let project = sqlx::query_as!(Project, r#"SELECT * FROM project WHERE id = ?"#, project_id)
        .fetch_one(pool)
        .await?;

    if project.visibility == "Private" {
        let user_id = get_user_id(session).await?;
        check_user_has_project_permission(user_id, project_id, &pool).await?;
    }

    Ok(project)
}

async fn get_projects_from_db(
    pool: &SqlitePool,
    session: &Session,
) -> Result<Vec<Project>, ProjectError> {
    let user_id = get_user_id(session).await;

    let projects = match user_id {
        Ok(user_id) => {
            sqlx::query_as!(Project, r#"
            SELECT id, environments, description, name, visibility FROM project
            LEFT JOIN project_admin ON project_admin.project_id = project.id AND project_admin.user_id = ?
            LEFT JOIN project_collaborator ON project_collaborator.project_id = project.id AND project_collaborator.user_id = ?
            WHERE visibility = "Public" OR project_admin.user_id = ? OR project_collaborator.user_id = ?;"#, user_id, user_id, user_id, user_id)
            .fetch_all(pool).await?
        },
        Err(_) => {
            sqlx::query_as!(
                Project,
                r#"SELECT * FROM project WHERE visibility = "Public""#
            )
            .fetch_all(pool)
            .await?
        }
    };

    Ok(projects)
}

async fn insert_project_into_db(
    project: &Project,
    pool: &SqlitePool,
    session: &Session,
) -> Result<i64, ProjectError> {
    let user_id = get_user_id(session).await?;

    let project_id = sqlx::query!(r#"INSERT INTO project (environments, description, name, visibility) VALUES (?, ?, ?, ?) RETURNING id"#, 
        project.environments, project.description, project.name, project.visibility)
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
        project.environments,
        project.description,
        project.name,
        project.visibility,
        project_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
