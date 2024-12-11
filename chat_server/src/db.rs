use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};

pub async fn db_init(db_path: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    if !std::path::Path::new(db_path).exists() {
        std::fs::File::create(db_path).expect("Failed to create database file");
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(32)
        .connect(&format!("sqlite:{}", db_path))
        .await
        .expect("Failed to connect database");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            group_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create messages table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS groups (
        id INTEGER PRIMARY KEY AUTOINCREMENT, 
        name TEXT NOT NULL,
        code TEXT NOT NULL UNIQUE COLLATE NOCASE,
        created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
    )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create groups table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create users table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS group_members (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            group_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            FOREIGN KEY (group_id) REFERENCES groups(id),
            FOREIGN KEY (user_id) REFERENCES users(id)
            ON DELETE CASCADE
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create group_members table");

    Ok(pool)
}
