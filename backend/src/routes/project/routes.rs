use crate::{
    helpers::get_user_id,
    models::{CurlGroup, Project},
    routes::project::types::{ProjectId, ProjectParams},
};

use actix_session::Session;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use sqlx::SqlitePool;

use super::types::ProjectError;

#[get("/project")]
#[tracing::instrument(name = "Getting projects.", skip(pool, session))]
async fn get_projects(
    query: web::Query<ProjectParams>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    // TODO: handle search query & labels
    match get_projects_from_db(&pool, &session).await {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().into(),
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
        Ok(id) => HttpResponse::Ok().json(ProjectId { id }),
        Err(_) => HttpResponse::InternalServerError().finish(),
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
        Err(_) => HttpResponse::InternalServerError().into(),
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
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/project/{project_id}")]
#[tracing::instrument(name = "Updating project.", skip(body, pool, session))]
async fn post_project(
    params: web::Path<i64>,
    body: web::Json<Project>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> impl Responder {
    match update_project_in_db(*params, &body, &pool, &session).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
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
        Ok(curl_group_id) => HttpResponse::Ok().json(curl_group_id),
        Err(_) => HttpResponse::InternalServerError().into(),
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
        Err(_) => HttpResponse::InternalServerError().into(),
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
        Err(_) => HttpResponse::InternalServerError().finish(),
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
    let user_id = get_user_id(session).await?;
    let project_id = sqlx::query!(
        "SELECT project_id FROM curl_group WHERE id = ?",
        curl_group_id
    )
    .fetch_one(pool)
    .await?
    .project_id;

    check_user_has_project_permission(user_id, project_id, pool).await?;

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

    if admin_query.is_err() && collaborator_query.is_err() {
        return Err(ProjectError::Unauthorised(
            "User does not have permission to view this cURL group".to_string(),
        ));
    }

    Ok(())
}

async fn check_user_has_project_admin_permission(
    user_id: i64,
    project_id: i64,
    pool: &SqlitePool,
) -> Result<(), ProjectError> {
    let query = sqlx::query!(
        r#"SELECT * FROM project_admin WHERE project_id = ? AND user_id = ?"#,
        project_id,
        user_id
    )
    .fetch_one(pool)
    .await;

    if query.is_err() {
        return Err(ProjectError::Unauthorised(
            "User does not have permission to for this project".to_string(),
        ));
    }

    Ok(())
}

async fn check_user_has_curl_group_permission(
    user_id: i64,
    group_id: i64,
    pool: &SqlitePool,
) -> Result<(), ProjectError> {
    let project_id = sqlx::query!(
        r#"SELECT project_id FROM curl_group WHERE id = ?"#,
        group_id
    )
    .fetch_one(pool)
    .await?
    .project_id;

    check_user_has_project_permission(user_id, project_id, pool).await
}

async fn get_curl_group_from_db(
    group_id: i64,
    pool: &SqlitePool,
    session: &Session,
) -> Result<CurlGroup, ProjectError> {
    let user_id = get_user_id(session).await?;
    check_user_has_curl_group_permission(user_id, group_id, &pool).await?;

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
    let _ = sqlx::query!(
        r#"DELETE FROM project_admin WHERE project_id = ?"#,
        project_id
    )
    .execute(pool)
    .await;
    let _ = sqlx::query!(
        r#"DELETE FROM project_collaborator WHERE project_id = ?"#,
        project_id
    )
    .execute(pool)
    .await;
    let _ = sqlx::query!(r#"DELETE FROM curl_group WHERE project_id = ?"#, project_id)
        .execute(pool)
        .await;

    Ok(())
}

async fn get_project_from_db(
    project_id: i64,
    pool: &SqlitePool,
    session: &Session,
) -> Result<Project, ProjectError> {
    let user_id = get_user_id(session).await?;
    check_user_has_project_permission(user_id, project_id, &pool).await?;

    let project = sqlx::query_as!(Project, r#"SELECT * FROM project WHERE id = ?"#, project_id)
        .fetch_one(pool)
        .await?;

    Ok(project)
}

async fn get_projects_from_db(
    pool: &SqlitePool,
    session: &Session,
) -> Result<Vec<Project>, ProjectError> {
    let user_id = get_user_id(session).await;

    let projects = match user_id {
        Ok(_user_id) => todo!("Not yet implemented"),
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
