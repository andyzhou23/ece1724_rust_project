use crate::jwt::create_jwt;
use crate::jwt::get_user_id;
use crate::AppConfig;
use actix_web::{post, web, HttpRequest, HttpResponse, Result};
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Row, Sqlite};
use std::collections::HashMap;
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
    app_config: web::Data<AppConfig>,
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
                "username": login_data.username,
                "access_token": create_jwt(user_id as usize, &app_config.jwt_secret).unwrap()
            })))
        }
        None => Ok(HttpResponse::Unauthorized().json(json!({
            "error": "Invalid credentials"
        }))),
    }
}

// ResponseHistory json[group_id, latest_msg_id]
#[derive(Deserialize)]
struct HistoryRequestEntry {
    group_id: usize,
    latest_msg_id: usize,
}
#[derive(Deserialize)]
struct HistoryRequest {
    entries: Vec<HistoryRequestEntry>,
}

#[post("/history")]
async fn get_history(
    pool: web::Data<Pool<Sqlite>>,
    req_body: web::Json<HistoryRequest>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = get_user_id(&req);
    let mut history = HashMap::new();
    for entry in req_body.entries.iter() {
        let membership = match sqlx::query(
            "SELECT user_id FROM group_members WHERE group_id = ? AND user_id = ?",
        )
        .bind(entry.group_id as i64)
        .bind(user_id as i64)
        .fetch_optional(pool.get_ref())
        .await
        {
            Ok(membership) => membership,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to check user membership"
                })))
            }
        };
        if membership.is_none() {
            println!(
                "User {} is not a member of group {}",
                user_id, entry.group_id
            );
            continue;
        }
        let messages = match sqlx::query(
            "SELECT m.id, m.group_id, m.user_id, m.content, m.created_at, u.name 
             FROM messages m
             JOIN users u ON m.user_id = u.id
             WHERE m.group_id = ? AND m.id > ? 
             ORDER BY m.id ASC",
        )
        .bind(entry.group_id as i64)
        .bind(entry.latest_msg_id as i64)
        .fetch_all(pool.get_ref())
        .await
        {
            Ok(rows) => rows
                .iter()
                .map(|row| {
                    json!({
                        "msg_id": row.get::<i64, _>("id"),
                        "group_id": row.get::<i64, _>("group_id"),
                        "sender_id": row.get::<i64, _>("user_id"),
                        "content": row.get::<String, _>("content"),
                        "created_at": row.get::<i64, _>("created_at"),
                        "sender_name": row.get::<String, _>("name")
                    })
                })
                .collect::<Vec<_>>(),
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to fetch messages"
                })));
            }
        }; // push the empty vec to show it's checked
        history.insert(entry.group_id, messages);
    }
    Ok(HttpResponse::Ok().json(history))
}
