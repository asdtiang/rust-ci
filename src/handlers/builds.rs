use askama_axum::IntoResponse;
use axum::{
    extract::{Path, Query, State},
    response::{Redirect, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, SeekFrom};
use tower_sessions::Session;

use crate::{
    models::{Build, Project},
    templates::{BuildDetailTemplate, DashboardBuild, DashboardTemplate},
    AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(dashboard))
        .route("/builds/:id", get(build_detail))
        .route("/api/builds/:id/logs", get(build_logs_api))
}

pub async fn dashboard(State(state): State<AppState>, session: Session) -> impl IntoResponse {
    let user_id: Option<i64> = session.get("user_id").await.unwrap_or(None);
    if user_id.is_none() {
        return Redirect::to("/login").into_response();
    }

    #[derive(sqlx::FromRow)]
    struct DashboardBuildRow {
        id: i64,
        project_name: String,
        status: String,
        started_at: Option<chrono::NaiveDateTime>,
        finished_at: Option<chrono::NaiveDateTime>,
    }

    let builds_res: Vec<DashboardBuildRow> = sqlx::query_as(
        r#"
        SELECT b.id, p.name as project_name, b.status, b.started_at, b.finished_at
        FROM builds b
        JOIN projects p ON b.project_id = p.id
        ORDER BY b.id DESC
        LIMIT 50
        "#
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let builds = builds_res
        .into_iter()
        .map(|row| DashboardBuild {
            id: row.id,
            project_name: row.project_name,
            status: row.status,
            started_at: row.started_at.map(|d| d.to_string()).unwrap_or_default(),
            finished_at: row.finished_at.map(|d| d.to_string()),
        })
        .collect();

    let lang = super::get_lang(&session).await;
    DashboardTemplate {
        lang,
        is_logged_in: true,
        error_msg: None,
        success_msg: None,
        builds,
    }
    .into_response()
}

pub async fn build_detail(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let user_id: Option<i64> = session.get("user_id").await.unwrap_or(None);
    if user_id.is_none() {
        return Redirect::to("/login").into_response();
    }

    let build: Option<Build> = sqlx::query_as("SELECT id, project_id, status, log_file, started_at, finished_at FROM builds WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None);

    let lang = super::get_lang(&session).await;
    if let Some(build) = build {
        let project: Project = sqlx::query_as("SELECT id, name, webhook_key, script_path, log_dir, created_at FROM projects WHERE id = ?")
            .bind(build.project_id)
            .fetch_one(&state.db)
            .await
            .unwrap();

        BuildDetailTemplate {
            lang,
            is_logged_in: true,
            error_msg: None,
            success_msg: None,
            build,
            project,
        }
        .into_response()
    } else {
        Redirect::to("/").into_response()
    }
}

#[derive(Deserialize)]
pub struct LogQuery {
    offset: u64,
}

#[derive(Serialize)]
pub struct LogResponse {
    text: String,
    new_offset: u64,
    status: String,
}

pub async fn build_logs_api(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(query): Query<LogQuery>,
) -> Json<LogResponse> {
    let build: Option<Build> = sqlx::query_as("SELECT id, project_id, status, log_file, started_at, finished_at FROM builds WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None);

    if let Some(build) = build {
        let mut text = String::new();
        let mut new_offset = query.offset;

        if let Ok(mut file) = std::fs::File::open(&build.log_file) {
            if query.offset == 0 {
                let start_offset = super::find_last_lines_offset(&mut file, 5000).unwrap_or(0);
                if start_offset > 0 {
                    text.push_str("... (skipping older logs, showing latest 5000 lines) ...\n");
                }
                let _ = file.seek(SeekFrom::Start(start_offset));
                new_offset = start_offset;
            } else {
                let _ = file.seek(SeekFrom::Start(query.offset));
            }

            if file.read_to_string(&mut text).is_ok() {
                new_offset += text.len() as u64;
            }
        }

        Json(LogResponse {
            text,
            new_offset,
            status: build.status,
        })
    } else {
        Json(LogResponse {
            text: String::new(),
            new_offset: query.offset,
            status: "unknown".to_string(),
        })
    }
}
