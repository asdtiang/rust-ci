use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::env;

pub async fn init_db() -> Result<SqlitePool> {
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:rust_ci.db".to_string());

    // Create DB file if it doesn't exist
    if !std::path::Path::new("rust_ci.db").exists() {
        std::fs::File::create("rust_ci.db")?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Create tables
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            is_admin BOOLEAN NOT NULL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            webhook_key TEXT NOT NULL UNIQUE,
            script_path TEXT NOT NULL,
            log_dir TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS builds (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            status TEXT NOT NULL,
            log_file TEXT NOT NULL,
            started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            finished_at DATETIME,
            FOREIGN KEY(project_id) REFERENCES projects(id)
        );
        "#,
    )
    .execute(&pool)
    .await?;

    // Seed admin user if not exists
    seed_admin(&pool).await?;

    Ok(pool)
}

async fn seed_admin(pool: &SqlitePool) -> Result<()> {
    let admin_exists: (i64,) = sqlx::query_as("SELECT count(*) FROM users WHERE username = 'admin'")
        .fetch_one(pool)
        .await?;

    if admin_exists.0 == 0 {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(b"admin456", &salt)
            .unwrap()
            .to_string();

        sqlx::query("INSERT INTO users (username, password_hash, is_admin) VALUES (?, ?, ?)")
            .bind("admin")
            .bind(password_hash)
            .bind(true)
            .execute(pool)
            .await?;
        
        tracing::info!("Created default admin user (admin / admin456)");
    }

    Ok(())
}
