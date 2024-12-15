use crate::chat::chat_server::ChatServer;
use crate::chat::messages::CheckUserStatus;
use crate::jwt::get_user_id;
use actix::prelude::*;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Result};
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Row, Sqlite};
use std::collections::HashMap;
#[derive(Deserialize)]
struct CreateGroupRequest {
    name: String,
}

fn generate_code(group_id: usize, unix_time: usize) -> String {
    let seed = group_id * 10000 + unix_time % 10000;
    format!("{:x}", seed).to_uppercase()
}

#[post("/group/create")]
async fn create_group(
    pool: web::Data<Pool<Sqlite>>,
    req_body: web::Json<CreateGroupRequest>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = get_user_id(&req);
    let group =
        match sqlx::query("INSERT INTO groups (name, code) VALUES (?, ?) RETURNING id, created_at")
            .bind(&req_body.name)
            .bind("TEMP")
            .fetch_one(pool.get_ref())
            .await
        {
            Ok(group) => group,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to create group"
                })));
            }
        };

    let group_id: i64 = group.get("id");
    let created_at: i64 = group.get("created_at");
    let code = generate_code(group_id as usize, created_at as usize);

    match sqlx::query("UPDATE groups SET code = ? WHERE id = ?")
        .bind(&code)
        .bind(group_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => (),
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to update group code"
            })));
        }
    };

    // Add creator as first group member
    match sqlx::query("INSERT INTO group_members (group_id, user_id) VALUES (?, ?)")
        .bind(group_id)
        .bind(user_id as i64)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => (),
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to add creator to group"
            })));
        }
    };

    Ok(HttpResponse::Ok().json(json!({
        "group_id": group_id,
        "group_name": req_body.name,
        "group_code": code
    })))
}

#[get("/group/list")]
async fn list_groups(pool: web::Data<Pool<Sqlite>>, req: HttpRequest) -> Result<HttpResponse> {
    let user_id = get_user_id(&req);
    // get groups that users in
    let groups = match sqlx::query(
        "SELECT g.id, g.name, g.code, g.created_at 
         FROM groups g
         INNER JOIN group_members gm ON g.id = gm.group_id
         WHERE gm.user_id = ?
         ORDER BY g.created_at DESC",
    )
    .bind(user_id as i64)
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(groups) => groups,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch groups"
            })));
        }
    };

    let mut groups_data = Vec::new();
    for group in groups {
        let group_id: i64 = group.get("id");

        // Get members for this group
        let members = match sqlx::query(
            "SELECT u.id, u.name 
             FROM users u
             INNER JOIN group_members gm ON u.id = gm.user_id
             WHERE gm.group_id = ?",
        )
        .bind(group_id)
        .fetch_all(pool.get_ref())
        .await
        {
            Ok(members) => members,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to fetch group members"
                })));
            }
        };

        let members_data: Vec<_> = members
            .iter()
            .map(|member| {
                json!({
                    "id": member.get::<i64, _>("id"),
                    "name": member.get::<String, _>("name")
                })
            })
            .collect();

        groups_data.push(json!({
            "id": group_id,
            "name": group.get::<String, _>("name"),
            "code": group.get::<String, _>("code"),
            "created_at": group.get::<i64, _>("created_at"),
            "members": members_data
        }));
    }

    Ok(HttpResponse::Ok().json(groups_data))
}

#[derive(Debug, Deserialize)]
struct JoinGroupRequest {
    group_code: String,
}

#[post("/group/join")]
async fn join_group(
    pool: web::Data<Pool<Sqlite>>,
    req_body: web::Json<JoinGroupRequest>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = get_user_id(&req);
    // Get group id from code
    let group = match sqlx::query("SELECT id, name, code FROM groups WHERE code = ?")
        .bind(&req_body.group_code)
        .fetch_optional(pool.get_ref())
        .await
    {
        Ok(group) => match group {
            Some(group) => group,
            None => {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "error": "Invalid group code"
                })));
            }
        },
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to check group code"
            })));
        }
    };

    let group_id: i64 = group.get("id");

    // Check if user is already a member
    let existing_member =
        match sqlx::query("SELECT id FROM group_members WHERE group_id = ? AND user_id = ?")
            .bind(group_id)
            .bind(user_id as i64)
            .fetch_optional(pool.get_ref())
            .await
        {
            Ok(member) => member,
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to check whether user is a member"
                })));
            }
        };

    if existing_member.is_some() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "User is already a member of this group"
        })));
    }

    // Add user to group
    match sqlx::query("INSERT INTO group_members (group_id, user_id) VALUES (?, ?)")
        .bind(group_id)
        .bind(user_id as i64)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "group_id": group_id,
            "group_name": group.get::<String, _>("name"),
            "group_code": group.get::<String, _>("code")
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": "Failed to join group"
        }))),
    }
}

#[derive(Deserialize)]
struct LeaveGroupRequest {
    group_id: i64,
}

#[post("/group/leave")]
pub async fn leave_group(
    req_body: web::Json<LeaveGroupRequest>,
    pool: web::Data<Pool<Sqlite>>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let user_id = get_user_id(&req);
    // Remove user from group
    match sqlx::query("DELETE FROM group_members WHERE group_id = ? AND user_id = ? RETURNING id")
        .bind(req_body.group_id)
        .bind(user_id as i64)
        .fetch_optional(pool.get_ref())
        .await
    {
        Ok(id) => match id {
            Some(_) => Ok(HttpResponse::Ok().json(json!({
                "message": format!("Successfully left group {}", req_body.group_id)
            }))),
            None => Ok(HttpResponse::BadRequest().json(json!({
                "error": format!("User is not a member of group {}", req_body.group_id)
            }))),
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": "Failed to leave group"
        }))),
    }
}

#[derive(Deserialize)]
struct GroupStatusRequest {
    group_id: i64,
}

#[get("/group/status")]
pub async fn group_status(
    req_body: web::Json<GroupStatusRequest>,
    pool: web::Data<Pool<Sqlite>>,
    req: HttpRequest,
    chat_server: web::Data<Addr<ChatServer>>,
) -> actix_web::Result<HttpResponse> {
    let user_id = get_user_id(&req);
    let membership =
        match sqlx::query("SELECT user_id FROM group_members WHERE group_id = ? AND user_id = ?")
            .bind(req_body.group_id)
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
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": format!("User {} is not a member of group {}", user_id, req_body.group_id)
        })));
    }
    let group_members = match sqlx::query("SELECT user_id FROM group_members WHERE group_id = ?")
        .bind(req_body.group_id)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(members) => members,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to retrieve group members"
            })));
        }
    };
    let member_ids: Vec<i64> = group_members.iter().map(|row| row.get("user_id")).collect();
    let mut online_members = HashMap::new();
    for id in member_ids {
        let username = chat_server
            .send(CheckUserStatus {
                user_id: id as usize,
            })
            .await;
        match username {
            Ok(username) => match username {
                Some(username) => {
                    online_members.insert(id as usize, username);
                }
                None => {
                    continue;
                }
            },
            Err(_) => {
                continue;
            }
        }
    }
    Ok(HttpResponse::Ok().json(json!({ "online_members": online_members })))
}
