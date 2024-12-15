use actix::prelude::*;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
// WebSocket Actor
use crate::chat::chat_server::ChatServer;
use crate::chat::connection_actor::ConnectionActor;
use crate::jwt::get_user_id;

#[get("/ws/connect")]
pub async fn ws_connect(
    req: HttpRequest,
    stream: web::Payload,
    chat_server: web::Data<Addr<ChatServer>>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, Error> {
    let user_id = get_user_id(&req);
    ws::start(
        ConnectionActor::new(
            chat_server.get_ref().clone(),
            user_id,
            app_config.heartbeat_timeout,
        ),
        &req,
        stream,
    )
}
use crate::jwt::validate_jwt;
use crate::AppConfig;
use serde_json::json;

#[get("/api/ws/connect/{access_token}")]
pub async fn ws_connect_with_path(
    req: HttpRequest,
    stream: web::Payload,
    chat_server: web::Data<Addr<ChatServer>>,
    access_token: web::Path<String>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, Error> {
    let user_id = match validate_jwt(&access_token.into_inner(), &app_config.jwt_secret) {
        Ok(claims) => claims.user_id,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "error": "Invalid token"
            })))
        }
    };
    ws::start(
        ConnectionActor::new(
            chat_server.get_ref().clone(),
            user_id,
            app_config.heartbeat_timeout,
        ),
        &req,
        stream,
    )
}
