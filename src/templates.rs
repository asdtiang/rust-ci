use askama::Template;
use crate::models::{Project, Build, User};
use crate::handlers::project_logs::LogFileInfo;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub lang: String,
    pub is_logged_in: bool,
    pub error_msg: Option<String>,
    pub success_msg: Option<String>,
}

pub struct DashboardBuild {
    pub id: i64,
    pub project_name: String,
    pub status: String,
    pub started_at: String,
    pub finished_at: Option<String>,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub lang: String,
    pub is_logged_in: bool,
    pub error_msg: Option<String>,
    pub success_msg: Option<String>,
    pub builds: Vec<DashboardBuild>,
}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {
    pub lang: String,
    pub is_logged_in: bool,
    pub error_msg: Option<String>,
    pub success_msg: Option<String>,
    pub projects: Vec<Project>,
}

#[derive(Template)]
#[template(path = "project_edit.html")]
pub struct ProjectEditTemplate {
    pub lang: String,
    pub is_logged_in: bool,
    pub error_msg: Option<String>,
    pub success_msg: Option<String>,
    pub project: Project,
}

#[derive(Template)]
#[template(path = "project_logs.html")]
pub struct ProjectLogsTemplate {
    pub lang: String,
    pub is_logged_in: bool,
    pub error_msg: Option<String>,
    pub success_msg: Option<String>,
    pub project: Project,
    pub log_files: Vec<LogFileInfo>,
}

#[derive(Template)]
#[template(path = "project_log_viewer.html")]
pub struct ProjectLogViewerTemplate {
    pub lang: String,
    pub is_logged_in: bool,
    pub error_msg: Option<String>,
    pub success_msg: Option<String>,
    pub project: Project,
    pub filename: String,
}

#[derive(Template)]
#[template(path = "build_detail.html")]
pub struct BuildDetailTemplate {
    pub lang: String,
    pub is_logged_in: bool,
    pub error_msg: Option<String>,
    pub success_msg: Option<String>,
    pub build: Build,
    pub project: Project,
}

#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersTemplate {
    pub lang: String,
    pub is_logged_in: bool,
    pub error_msg: Option<String>,
    pub success_msg: Option<String>,
    pub users: Vec<User>,
}
