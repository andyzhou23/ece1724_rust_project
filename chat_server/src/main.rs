use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};

use serde::{Deserialize};
use serde_json::json;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Row, Sqlite};
// use std::sync::{
//     Arc,
// };

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Rust-powered chat server!")
}

#[derive(Debug, Deserialize)]
struct UserCredentials {
    username: String,
    password: String,
}

#[post("/signup")]
async fn signup(
    pool: web::Data<Pool<Sqlite>>,
    signup_data: web::Json<UserCredentials>,
) -> Result<HttpResponse> {
    // Check if username already exists
    let existing_user = match sqlx::query("SELECT id FROM users WHERE name = ?")
        .bind(&signup_data.username)
        .fetch_optional(pool.get_ref())
        .await
    {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to check username availability"
            })));
        }
    };

    if existing_user.is_some() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "Username already taken"
        })));
    }

    // Insert new user
    let result = match sqlx::query("INSERT INTO users (name, password) VALUES (?, ?)")
        .bind(&signup_data.username)
        .bind(&signup_data.password) // todo: hash
        .execute(pool.get_ref())
        .await
    {
        Ok(result) => result,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to create user"
            })));
        }
    };

    let user_id = result.last_insert_rowid();

    Ok(HttpResponse::Ok().json(json!({
        "id": user_id,
        "username": signup_data.username
    })))
}

#[post("/login")]
async fn login(
    pool: web::Data<Pool<Sqlite>>,
    login_data: web::Json<UserCredentials>,
) -> Result<HttpResponse> {
    // Check if user exists and password matches
    let user = match sqlx::query("SELECT id, password FROM users WHERE name = ?")
        .bind(&login_data.username)
        .fetch_optional(pool.get_ref())
        .await
    {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to check user credentials"
            })));
        }
    };

    match user {
        Some(row) => {
            let stored_password: String = row.get("password");
            if stored_password != login_data.password {
                return Ok(HttpResponse::Unauthorized().json(json!({
                    "error": "Invalid credentials"
                })));
            }

            let user_id: i64 = row.get("id");
            Ok(HttpResponse::Ok().json(json!({
                "id": user_id,
                "username": login_data.username
            })))
        }
        None => Ok(HttpResponse::Unauthorized().json(json!({
            "error": "Invalid credentials"
        }))),
    }
}

// #[get("/count")]
// async fn count(visit_count: web::Data<Arc<AtomicU64>>) -> impl Responder {
//     let count = visit_count.fetch_add(1, Ordering::Relaxed) + 1;
//     HttpResponse::Ok().body(format!("Visit count: {}", count))
// }

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

    println!("The server is currently listening on localhost:8080.");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(signup)
            .service(login)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
