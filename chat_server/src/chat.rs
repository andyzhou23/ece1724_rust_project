// actor messages

use actix::prelude::*;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::Deserialize;
use serde_json::{self, json};
use sqlx::{Pool, Row, Sqlite};
use std::collections::HashMap;
use std::time::{Duration, Instant};
// Actor Messages

#[derive(Message)]
#[rtype(result = "()")]
struct AddSession {
    user_id: usize,
    addr: Addr<ConnectionActor>,
}

#[derive(Message)]
#[rtype(result = "()")]
struct RemoveSession {
    user_id: usize,
}

// WebSocket Actor
struct ConnectionActor {
    user_id: usize,
    chat_server: Addr<ChatServer>,
    last_active_at: Instant,
}

impl ConnectionActor {
    fn new(chat_server: Addr<ChatServer>, user_id: usize) -> Self {
        Self {
            user_id,
            chat_server,
            last_active_at: Instant::now(),
        }
    }

    fn start_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(5), |actor, ctx| {
            if Instant::now().duration_since(actor.last_active_at) > Duration::from_secs(100000) {
                ctx.stop(); // todo: config heartbeat timeout
            }
        });
    }
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
            .then(|res, actor, _ctx| {
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

#[derive(Message, Deserialize)]
#[rtype(result = "()")]
struct ClientMessage {
    user_id: usize,
    group_id: usize,
    content: String,
}

#[derive(Deserialize)]
struct ClientMessageJson {
    group_id: usize,
    content: String,
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
            }
            Ok(ws::Message::Ping(msg)) => {
                self.last_active_at = Instant::now();
                ctx.pong(&msg);
            }
            _ => ctx.stop(), // including close
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct BroadcastMessage {
    msg_id: usize,
    user_id: usize,
    group_id: usize,
    content: String,
    created_at: u64,
}

impl Handler<BroadcastMessage> for ConnectionActor {
    // send message to client
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        let payload = json!({
            "msg_id": msg.msg_id,
            "user_id": msg.user_id,
            "group_id": msg.group_id,
            "content": msg.content,
            "created_at": msg.created_at
        });
        ctx.text(payload.to_string());
    }
}

#[derive(Clone)]
struct SessionInfo {
    user_id: usize,
    username: String,
    addr: Addr<ConnectionActor>,
    connected_at: Instant,
}

#[derive(Clone)]
pub struct ChatServer {
    pool: Pool<Sqlite>,
    sessions: HashMap<usize, SessionInfo>,
    created_at: Instant,
}

impl ChatServer {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            sessions: HashMap::new(),
            created_at: Instant::now(),
        }
    }

    async fn add_session(&mut self, addr: Addr<ConnectionActor>, user_id: usize) {
        let user_entry = match sqlx::query("SELECT name FROM users WHERE id = ?")
            .bind(user_id as i64)
            .fetch_optional(&self.pool)
            .await
        {
            Ok(user) => user,
            Err(_) => return,
        };

        let username = user_entry.unwrap().get("name");

        self.sessions.insert(
            user_id,
            SessionInfo {
                user_id,
                username,
                addr,
                connected_at: Instant::now(),
            },
        );
    }

    fn remove_session(&mut self, user_id: usize) {
        self.sessions.remove(&user_id);
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        // todo: broad cast message to all connected clients in group
        for (_, session) in self.sessions.iter_mut() {
            session.addr.do_send(BroadcastMessage {
                msg_id: 0,
                user_id: msg.user_id,
                group_id: msg.group_id,
                content: msg.content.clone(),
                created_at: 0,
            });
        }
    }
}

impl Handler<AddSession> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: AddSession, ctx: &mut Context<Self>) {
        let addr = msg.addr;
        let user_id = msg.user_id;
        let addr_clone = addr.clone();
        let user_id_clone = user_id;
        let mut self_clone = self.clone();

        actix::spawn(async move {
            self_clone.add_session(addr_clone, user_id_clone).await;
        });
    }
}

impl Handler<RemoveSession> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: RemoveSession, _: &mut Context<Self>) {
        self.remove_session(msg.user_id);
    }
}

#[get("/ws/connect")]
pub async fn ws_connect(
    // ws://localhost:8080/ws/connect
    req: HttpRequest,
    stream: web::Payload,
    // user_id: web::Query<usize>,
    // pool: web::Data<Pool<Sqlite>>,
    chat_server: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    println!("ws_connect");
    // todo credential check
    // let user_id = user_id.into_inner();
    let user_id = 1;
    ws::start(
        ConnectionActor::new(chat_server.get_ref().clone(), user_id),
        &req,
        stream,
    )
}

#[get("/ws/test")]
async fn ws_test() -> HttpResponse {
    HttpResponse::Ok().body("WebSocket test endpoint")
}
