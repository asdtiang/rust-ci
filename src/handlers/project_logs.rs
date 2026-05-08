use askama_axum::IntoResponse;
use axum::{
    extract::{Path, Query, State},
    response::{Json, Redirect},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, SeekFrom};
use tower_sessions::Session;

use crate::{
    models::Project,
    templates::{ProjectLogViewerTemplate, ProjectLogsTemplate},
    AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/projects/:id/logs", get(project_logs_list))
        .route("/projects/:id/logs/:filename", get(project_log_viewer))
        .route("/api/projects/:id/logs/:filename", get(project_log_api))
}

pub struct LogFileInfo {
    pub name: String,
    pub size: u64,
}

pub async fn project_logs_list(
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

    if let Some(project) = project {
        let mut log_files = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&project.log_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        log_files.push(LogFileInfo {
                            name,
                            size: metadata.len(),
                        });
                    }
                }
            }
        }

        log_files.sort_by(|a, b| a.name.cmp(&b.name));

        let lang = super::get_lang(&session).await;
        ProjectLogsTemplate {
            lang,
            is_logged_in: true,
            error_msg: None,
            success_msg: None,
            project,
            log_files,
        }
        .into_response()
    } else {
        Redirect::to("/projects").into_response()
    }
}

pub async fn project_log_viewer(
    State(state): State<AppState>,
    session: Session,
    Path((id, filename)): Path<(i64, String)>,
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
        ProjectLogViewerTemplate {
            lang,
            is_logged_in: true,
            error_msg: None,
            success_msg: None,
            project,
            filename,
        }
        .into_response()
    } else {
        Redirect::to("/projects").into_response()
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
}

pub async fn project_log_api(
    State(state): State<AppState>,
    Path((id, filename)): Path<(i64, String)>,
    Query(query): Query<LogQuery>,
) -> Json<LogResponse> {
    let project: Option<Project> = sqlx::query_as("SELECT id, name, webhook_key, script_path, log_dir, created_at FROM projects WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None);

    if let Some(project) = project {
        // Security check: ensure filename doesn't contain path traversal
        if filename.contains('/') || filename.contains('\\') || filename.contains("..") {
            return Json(LogResponse {
                text: "Invalid filename".to_string(),
                new_offset: query.offset,
            });
        }

        let file_path = std::path::Path::new(&project.log_dir).join(&filename);
        let mut text = String::new();
        let mut new_offset = query.offset;

        if let Ok(mut file) = std::fs::File::open(&file_path) {
            if query.offset == 0 {
                // Initial load: get last 5000 lines
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

        Json(LogResponse { text, new_offset })
    } else {
        Json(LogResponse {
            text: String::new(),
            new_offset: query.offset,
        })
    }
}
