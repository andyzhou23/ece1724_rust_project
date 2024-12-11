use actix::prelude::*;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
// WebSocket Actor
use crate::chat::chat_server::ChatServer;
use crate::chat::connection_actor::ConnectionActor;

#[get("/ws/connect/{user_id}")]
pub async fn ws_connect(
    req: HttpRequest,
    stream: web::Payload,
    user_id: web::Path<usize>,
    chat_server: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    // todo credential check
    let user_id = user_id.into_inner();
    ws::start(
        ConnectionActor::new(chat_server.get_ref().clone(), user_id),
        &req,
        stream,
    )
}
