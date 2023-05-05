use actix::prelude::{Message, Recipient};
use serde::{Deserialize, Deserializer};
use crate::repository::message::Message as RepositoryMessage;

//WsConn responds to this to pipe it through to the actual client
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

//WsConn sends this to the lobby to say "put me in please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub lobby_id: i64,
    pub user_id: i64,
}

//WsConn sends this to a lobby to say "take me out please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub room_id: i64,
    pub user_id: i64,
}

//client sends this to the lobby for the lobby to echo out.
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub user_id: i64,
    pub msg: String,
    pub room_id: i64
}