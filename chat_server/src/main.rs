use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sqlx::sqlite::SqlitePoolOptions;

mod auth;
use auth::{login, signup};

mod ws;
use ws::{ws_connect, ws_test, ChatServer};
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Rust-powered chat server!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_path = "server.db";
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
        code TEXT NOT NULL UNIQUE,
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
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create group_members table");

    let chat_server = ChatServer::new(pool.clone());

    println!("The server is currently listening on localhost:8080.");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(chat_server.clone()))
            .service(index)
            .service(signup)
            .service(login)
            .service(ws_connect)
            .service(ws_test)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
