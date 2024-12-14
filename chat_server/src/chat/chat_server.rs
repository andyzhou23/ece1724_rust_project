use actix::prelude::*;
use sqlx::{Pool, Row, Sqlite};
use std::collections::HashMap;
use std::time::Instant;

use crate::chat::connection_actor::ConnectionActor;
use crate::chat::messages::{AddSession, BroadcastMessage, ClientMessage, RemoveSession};
#[derive(Clone)]
struct SessionInfo {
    _user_id: usize,
    _username: String,
    addr: Addr<ConnectionActor>,
    _connected_at: Instant,
}

#[derive(Clone)]
pub struct ChatServer {
    pool: Pool<Sqlite>,
    sessions: HashMap<usize, SessionInfo>,
    _created_at: Instant,
}

impl ChatServer {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            sessions: HashMap::new(),
            _created_at: Instant::now(),
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
                _user_id: user_id,
                _username: username,
                addr,
                _connected_at: Instant::now(),
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
        let is_member = futures::executor::block_on(
            sqlx::query("SELECT 1 FROM group_members WHERE group_id = ? AND user_id = ?")
                .bind(msg.group_id as i64)
                .bind(msg.user_id as i64)
                .fetch_optional(&self.pool),
        )
        .is_ok();

        if !is_member {
            println!(
                "User {} is not a member of group {}",
                msg.user_id, msg.group_id
            );
            return;
        }
        // Save message to database
        let insert_result = futures::executor::block_on(
            sqlx::query("INSERT INTO messages (group_id, user_id, content) VALUES (?, ?, ?) RETURNING id, created_at")
                .bind(msg.group_id as i64)
                .bind(msg.user_id as i64)
                .bind(&msg.content)
                .fetch_one(&self.pool),
        );

        let (msg_id, created_at) = match insert_result {
            Ok(result) => (
                result.get::<i64, _>("id") as usize,
                result.get::<i64, _>("created_at") as u64,
            ),
            Err(e) => {
                println!("Failed to save message: {}", e);
                return;
            }
        };

        let sender_name = match futures::executor::block_on(
            sqlx::query("SELECT name FROM users WHERE id = ?")
                .bind(msg.user_id as i64)
                .fetch_one(&self.pool),
        ) {
            Ok(result) => result.get::<String, _>("name"),
            Err(e) => {
                println!("Failed to get sender name: {}", e);
                return;
            }
        };

        // Get users in the same group
        let target_users = match futures::executor::block_on(
            sqlx::query("SELECT user_id FROM group_members WHERE group_id = ?")
                .bind(msg.group_id as i64)
                .fetch_all(&self.pool),
        ) {
            Ok(users) => users,
            Err(e) => {
                println!("Failed to get group members: {}", e);
                return;
            }
        };

        // Only broadcast to users in the same group
        for row in target_users {
            let target_user_id: i64 = row.get("user_id");
            if let Some(session) = self.sessions.get(&(target_user_id as usize)) {
                session.addr.do_send(BroadcastMessage {
                    msg_id,
                    sender_id: msg.user_id,
                    group_id: msg.group_id,
                    content: msg.content.clone(),
                    created_at,
                    sender_name: sender_name.clone(),
                });
            }
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
