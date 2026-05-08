use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::post,
    Router,
};
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use crate::{models::Project, AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/webhook/:key", post(webhook_trigger))
}

pub async fn webhook_trigger(State(state): State<AppState>, Path(key): Path<String>) -> impl IntoResponse {
    let project: Option<Project> = sqlx::query_as("SELECT id, name, webhook_key, script_path, log_dir, created_at FROM projects WHERE webhook_key = ?")
        .bind(&key)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None);

    if let Some(project) = project {
        // Ensure ci_log dir exists
        let _ = tokio::fs::create_dir_all("ci_log").await;

        let log_file_path = format!("ci_log/{}.log", uuid::Uuid::new_v4());

        // Create initial build record
        let build_id = sqlx::query("INSERT INTO builds (project_id, status, log_file) VALUES (?, 'running', ?)")
            .bind(project.id)
            .bind(&log_file_path)
            .execute(&state.db)
            .await
            .unwrap()
            .last_insert_rowid();

        // Spawn async task
        let db = state.db.clone();
        tokio::spawn(async move {
            let file_res = tokio::fs::File::create(&log_file_path).await;
            let mut file = match file_res {
                Ok(f) => f,
                Err(e) => {
                    let _ = sqlx::query("UPDATE builds SET status = 'failed', finished_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(build_id)
                        .execute(&db)
                        .await;
                    tracing::error!("Failed to create log file: {}", e);
                    return;
                }
            };
            let _ = file.write_all(b"Build started...\n").await;

            let child_res = Command::new("sh")
                .arg("-c")
                .arg(&project.script_path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn();

            let mut child = match child_res {
                Ok(c) => c,
                Err(e) => {
                    let _ = file.write_all(format!("\nFailed to spawn script: {}\n", e).as_bytes()).await;
                    let _ = sqlx::query("UPDATE builds SET status = 'failed', finished_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(build_id)
                        .execute(&db)
                        .await;
                    return;
                }
            };

            // Stream output to file
            let mut stdout = child.stdout.take().unwrap();
            let mut stderr = child.stderr.take().unwrap();

            let log_file_path_clone = log_file_path.clone();
            let stdout_task = tokio::spawn(async move {
                if let Ok(mut f) = tokio::fs::OpenOptions::new().append(true).open(&log_file_path_clone).await {
                    let _ = tokio::io::copy(&mut stdout, &mut f).await;
                }
            });

            let log_file_path_clone2 = log_file_path.clone();
            let stderr_task = tokio::spawn(async move {
                if let Ok(mut f) = tokio::fs::OpenOptions::new().append(true).open(&log_file_path_clone2).await {
                    let _ = tokio::io::copy(&mut stderr, &mut f).await;
                }
            });

            let status = child.wait().await.unwrap();
            let _ = stdout_task.await;
            let _ = stderr_task.await;

            let final_status = if status.success() {
                "success"
            } else {
                "failed"
            };

            let _ = file.write_all(format!("\nBuild finished with status: {}\n", final_status).as_bytes()).await;

            sqlx::query("UPDATE builds SET status = ?, finished_at = CURRENT_TIMESTAMP WHERE id = ?")
                .bind(final_status)
                .bind(build_id)
                .execute(&db)
                .await
                .unwrap();
        });

        "Build triggered successfully"
    } else {
        "Invalid webhook key"
    }
}
