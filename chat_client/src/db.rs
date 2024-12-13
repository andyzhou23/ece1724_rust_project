use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use std::path::Path;
use std::fs::File;



pub async fn db_init(db_path: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    // Check if the database file exists, and create it if not
    if !Path::new(db_path).exists() {
        File::create(db_path).expect("Failed to create database file");
    }

    // Create a connection pool to the SQLite database
    let pool = SqlitePoolOptions::new()
        .max_connections(32)
        .connect(&format!("sqlite:{}", db_path))
        .await?;

    // Create the 'messages' table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY,
            group_id INTEGER NOT NULL,
            sender_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    Ok(pool) // Return the pool
}


//test insert message s
pub struct BroadcastMessage {
    pub msg_id: usize,
    pub sender_id: usize,
    pub group_id: usize,
    pub content: String,
    pub created_at: u64,
}

async fn insert_message(pool: &Pool<Sqlite>, message: &BroadcastMessage) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO messages (id, group_id, sender_id, content, created_at) 
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(message.msg_id as i64)
    .bind(message.group_id as i64)
    .bind(message.sender_id as i64)
    .bind(&message.content)
    .bind(message.created_at as i64)
    .execute(pool)
    .await?;

    Ok(())
}