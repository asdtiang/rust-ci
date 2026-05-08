use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    response::Redirect,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use tower_sessions::Session;
use uuid::Uuid;

use crate::{models::Project, templates::{ProjectsTemplate, ProjectEditTemplate}, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/projects", get(projects_list).post(projects_post))
        .route("/projects/:id/edit", get(project_edit_form).post(project_edit_post))
}

pub async fn projects_list(State(state): State<AppState>, session: Session) -> impl IntoResponse {
    let user_id: Option<i64> = session.get("user_id").await.unwrap_or(None);
    if user_id.is_none() {
        return Redirect::to("/login").into_response();
    }

    let projects: Vec<Project> = sqlx::query_as(
        "SELECT id, name, webhook_key, script_path, log_dir, created_at FROM projects",
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let lang = super::get_lang(&session).await;
    ProjectsTemplate {
        lang,
        is_logged_in: true,
        error_msg: None,
        success_msg: None,
        projects,
    }
    .into_response()
}

#[derive(Deserialize)]
pub struct ProjectPayload {
    pub name: String,
    pub script_path: String,
    pub log_dir: String,
}

pub async fn projects_post(
    State(state): State<AppState>,
    session: Session,
    Form(payload): Form<ProjectPayload>,
) -> impl IntoResponse {
    let user_id: Option<i64> = session.get("user_id").await.unwrap_or(None);
    if user_id.is_none() {
        return Redirect::to("/login").into_response();
    }

    let webhook_key = Uuid::new_v4().to_string();

    let res = sqlx::query(
        "INSERT INTO projects (name, webhook_key, script_path, log_dir) VALUES (?, ?, ?, ?)",
    )
    .bind(&payload.name)
    .bind(webhook_key)
    .bind(&payload.script_path)
    .bind(&payload.log_dir)
    .execute(&state.db)
    .await;

    let projects: Vec<Project> = sqlx::query_as(
        "SELECT id, name, webhook_key, script_path, log_dir, created_at FROM projects",
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let lang = super::get_lang(&session).await;
    match res {
        Ok(_) => ProjectsTemplate {
            lang: lang.clone(),
            is_logged_in: true,
            error_msg: None,
            success_msg: Some(crate::i18n::t("proj_saved", &lang).to_string()),
            projects,
        }
        .into_response(),
        Err(e) => ProjectsTemplate {
            lang,
            is_logged_in: true,
            error_msg: Some(format!("Error: {}", e)),
            success_msg: None,
            projects,
        }
        .into_response(),
    }
}

pub async fn project_edit_form(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let user_id: Option<i64> = session.get("user_id").await.unwrap_or(None);
    if user_id.is_none() {
        return Redirect::to("/login").into_response();
    }

    let project: Option<Project> = sqlx::query_as("SELECT id, name, webhook_key, script_path, log_dir, created_at FROM projects WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None);

    let lang = super::get_lang(&session).await;
    if let Some(project) = project {
        ProjectEditTemplate {
            lang,
            is_logged_in: true,
            error_msg: None,
            success_msg: None,
            project,
        }
        .into_response()
    } else {
        Redirect::to("/projects").into_response()
    }
}

pub async fn project_edit_post(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i64>,
    Form(payload): Form<ProjectPayload>,
) -> impl IntoResponse {
    let user_id: Option<i64> = session.get("user_id").await.unwrap_or(None);
    if user_id.is_none() {
        return Redirect::to("/login").into_response();
    }

    let res = sqlx::query(
        "UPDATE projects SET name = ?, script_path = ?, log_dir = ? WHERE id = ?"
    )
    .bind(&payload.name)
    .bind(&payload.script_path)
    .bind(&payload.log_dir)
    .bind(id)
    .execute(&state.db)
    .await;

    // Fetch projects again for the project list page
    let projects: Vec<Project> = sqlx::query_as(
        "SELECT id, name, webhook_key, script_path, log_dir, created_at FROM projects",
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let lang = super::get_lang(&session).await;
    match res {
        Ok(_) => ProjectsTemplate {
            lang: lang.clone(),
            is_logged_in: true,
            error_msg: None,
            success_msg: Some(crate::i18n::t("proj_saved", &lang).to_string()),
            projects,
        }
        .into_response(),
        Err(e) => ProjectsTemplate {
            lang,
            is_logged_in: true,
            error_msg: Some(format!("Error: {}", e)),
            success_msg: None,
            projects,
        }
        .into_response(),
    }
}
