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
) -> Result<HttpResponse, Error> {
    let user_id = get_user_id(&req);
    ws::start(
        ConnectionActor::new(chat_server.get_ref().clone(), user_id),
        &req,
        stream,
    )
}
