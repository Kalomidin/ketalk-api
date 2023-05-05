use actix::prelude::{Message, Recipient};
use serde::{Deserialize, Serialize};
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

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct ClientWsMessage {
    pub message_type: ClientWsMessageType,
    pub data: String
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub enum ClientWsMessageType {
    Message,
    Editing,
    MessageDelete,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct UserRooms {
    pub rooms: Vec<UserRoom>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct UserRoom {
    pub room_name: String,
    pub last_message: String,
    pub last_message_time: String,
    pub last_message_sender_id: i64,
}
