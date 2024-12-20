use actix::prelude::*;
use serde::Serialize;
// Actor Messages
use crate::chat::connection_actor::ConnectionActor;
#[derive(Message)]
#[rtype(result = "()")]
pub struct AddSession {
    pub user_id: usize,
    pub addr: Addr<ConnectionActor>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RemoveSession {
    pub user_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Serialize)]
pub struct ClientMessage {
    pub user_id: usize,
    pub group_id: usize,
    pub content: String,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Serialize)]
pub struct BroadcastMessage {
    pub msg_id: usize,
    pub sender_id: usize,
    pub group_id: usize,
    pub content: String,
    pub created_at: u64,
    pub sender_name: String,
}

#[derive(Message)]
#[rtype(result = "Option<String>")]
pub struct CheckUserStatus {
    pub user_id: usize,
}
