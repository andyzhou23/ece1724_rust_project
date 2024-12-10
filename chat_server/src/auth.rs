use actix_web::{post, web, HttpResponse, Result};
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Row, Sqlite};

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
