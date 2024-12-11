use actix::prelude::*;
use sqlx::{Pool, Row, Sqlite};
use std::collections::HashMap;
use std::time::Instant;

use crate::chat::connection_actor::ConnectionActor;
use crate::chat::messages::{AddSession, BroadcastMessage, ClientMessage, RemoveSession};
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

    fn add_session(&mut self, addr: Addr<ConnectionActor>, user_id: usize) {
        let user_entry = match futures::executor::block_on(
            sqlx::query("SELECT name FROM users WHERE id = ?")
                .bind(user_id as i64)
                .fetch_optional(&self.pool),
        ) {
            Ok(user) => user,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        let username = match user_entry {
            Some(entry) => entry.get("name"),
            None => {
                println!("User {} not found", user_id);
                return;
            }
        };

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
                sender_id: msg.user_id,
                group_id: msg.group_id,
                content: msg.content.clone(),
                created_at: 0,
            });
        }
    }
}

impl Handler<AddSession> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: AddSession, _: &mut Context<Self>) {
        let addr = msg.addr;
        let user_id = msg.user_id;
        self.add_session(addr, user_id);
    }
}

impl Handler<RemoveSession> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: RemoveSession, _: &mut Context<Self>) {
        self.remove_session(msg.user_id);
    }
}
