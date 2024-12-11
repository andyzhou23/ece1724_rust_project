use actix::prelude::*;
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
pub struct ClientMessage {
    pub user_id: usize,
    pub group_id: usize,
    pub content: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastMessage {
    pub msg_id: usize,
    pub sender_id: usize,
    pub group_id: usize,
    pub content: String,
    pub created_at: u64,
}
