use crate::chat::chat_server::ChatServer;
use crate::chat::messages::{AddSession, BroadcastMessage, ClientMessage, RemoveSession};
use actix::prelude::*;
use actix_web_actors::ws;
use serde::Deserialize;
use serde_json::{self, json};
use std::time::{Duration, Instant};

pub struct ConnectionActor {
    user_id: usize,
    chat_server: Addr<ChatServer>,
    last_active_at: Instant,
}

impl ConnectionActor {
    pub fn new(chat_server: Addr<ChatServer>, user_id: usize) -> Self {
        Self {
            user_id,
            chat_server,
            last_active_at: Instant::now(),
        }
    }

    fn start_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(5), |actor, ctx| {
            if Instant::now().duration_since(actor.last_active_at) > Duration::from_secs(10) {
                ctx.stop(); // todo: config heartbeat timeout
            }
        });
    }
}
#[derive(Deserialize)]
struct ClientMessageJson {
    group_id: usize,
    content: String,
}

impl Actor for ConnectionActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // on connection start
        self.start_heartbeat(ctx);
        self.chat_server
            .send(AddSession {
                user_id: self.user_id,
                addr: ctx.address(),
            })
            .into_actor(self)
            .then(|res, _actor, _ctx| {
                match res {
                    Ok(_) => (),
                    _ => println!("Failed to add session"),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        self.chat_server.do_send(RemoveSession {
            user_id: self.user_id,
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ConnectionActor {
    // handle message from client
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let payload: ClientMessageJson = match serde_json::from_str(&text) {
                    Ok(payload) => payload,
                    Err(_) => ClientMessageJson {
                        group_id: 0,
                        content: "RawText: ".to_string() + &text.to_string(),
                    },
                };
                self.chat_server.do_send(ClientMessage {
                    user_id: self.user_id,
                    group_id: payload.group_id,
                    content: payload.content,
                });
                self.last_active_at = Instant::now();
            }
            Ok(ws::Message::Ping(msg)) => {
                self.last_active_at = Instant::now();
                ctx.pong(&msg);
            }
            _ => ctx.stop(), // including close
        }
    }
}

impl Handler<BroadcastMessage> for ConnectionActor {
    // send message to client
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        let payload = json!({
            "msg_id": msg.msg_id,
            "sender_id": msg.sender_id,
            "group_id": msg.group_id,
            "content": msg.content,
            "created_at": msg.created_at
        });
        ctx.text(payload.to_string());
    }
}
