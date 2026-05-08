use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use askama_axum::IntoResponse;
use axum::{
    extract::State,
    response::Redirect,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{
    models::User,
    templates::{LoginTemplate, UsersTemplate},
    AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(login_form).post(login_post))
        .route("/logout", post(logout))
        .route("/users", get(users_list).post(users_post))
        .route("/lang", post(switch_lang))
}

pub async fn login_form(session: Session) -> impl IntoResponse {
    let lang = super::get_lang(&session).await;
    LoginTemplate {
        lang,
        is_logged_in: false,
        error_msg: None,
        success_msg: None,
    }
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

pub async fn login_post(
    State(state): State<AppState>,
    session: Session,
    Form(payload): Form<LoginPayload>,
) -> impl IntoResponse {
    let result: Result<User, sqlx::Error> = sqlx::query_as(
        "SELECT id, username, password_hash, is_admin, created_at FROM users WHERE username = ?",
    )
    .bind(&payload.username)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(user) => {
            let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
            if Argon2::default()
                .verify_password(payload.password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                session.insert("user_id", user.id).await.unwrap();
                return Redirect::to("/").into_response();
            }
        }
        Err(_) => {}
    }

    let lang = super::get_lang(&session).await;
    let error_msg = Some(crate::i18n::t("login_err", &lang).to_string());
    LoginTemplate {
        lang,
        is_logged_in: false,
        error_msg,
        success_msg: None,
    }
    .into_response()
}

#[derive(Deserialize)]
pub struct LangPayload {
    pub lang: String,
    pub next: String,
}

pub async fn switch_lang(session: Session, Form(payload): Form<LangPayload>) -> impl IntoResponse {
    session.insert("lang", payload.lang).await.unwrap();
    Redirect::to(&payload.next)
}

pub async fn logout(session: Session) -> impl IntoResponse {
    session.clear().await;
    Redirect::to("/login")
}

#[derive(Deserialize)]
pub struct UserPayload {
    pub username: String,
    pub password: String,
    pub is_admin: Option<String>,
}

pub async fn users_list(State(state): State<AppState>, session: Session) -> impl IntoResponse {
    let user_id: Option<i64> = session.get("user_id").await.unwrap_or(None);
    if user_id.is_none() {
        return Redirect::to("/login").into_response();
    }

    let users: Vec<User> = sqlx::query_as("SELECT id, username, password_hash, is_admin, created_at FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    let lang = super::get_lang(&session).await;
    UsersTemplate {
        lang,
        is_logged_in: true,
        error_msg: None,
        success_msg: None,
        users,
    }
    .into_response()
}

pub async fn users_post(
    State(state): State<AppState>,
    session: Session,
    Form(payload): Form<UserPayload>,
) -> impl IntoResponse {
    let user_id: Option<i64> = session.get("user_id").await.unwrap_or(None);
    if user_id.is_none() {
        return Redirect::to("/login").into_response();
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let is_admin = payload.is_admin.is_some();

    let res = sqlx::query(
        "INSERT INTO users (username, password_hash, is_admin) VALUES (?, ?, ?)
         ON CONFLICT(username) DO UPDATE SET password_hash = excluded.password_hash, is_admin = excluded.is_admin",
    )
    .bind(&payload.username)
    .bind(password_hash)
    .bind(is_admin)
    .execute(&state.db)
    .await;

    let users: Vec<User> = sqlx::query_as("SELECT id, username, password_hash, is_admin, created_at FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    let lang = super::get_lang(&session).await;
    match res {
        Ok(_) => UsersTemplate {
            lang: lang.clone(),
            is_logged_in: true,
            error_msg: None,
            success_msg: Some(crate::i18n::t("users_saved", &lang).to_string()),
            users,
        }.into_response(),
        Err(e) => UsersTemplate {
            lang,
            is_logged_in: true,
            error_msg: Some(format!("Error: {}", e)),
            success_msg: None,
            users,
        }.into_response()
    }
}
