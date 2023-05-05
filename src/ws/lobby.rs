use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};

use super::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use crate::routes::DbPool;
use crate::repository::message::get_messages_for_room_id;
use crate::repository::message::create_new_message_from_insert_struct;
use crate::repository::message::InsertMessage;
use crate::helpers::new_naive_date;

pub type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<i64, Socket>,          //self id to self
    rooms: HashMap<i64, HashSet<i64>>,      //room id  to list of session ids
    pool: DbPool,
}

impl Lobby {
    pub fn new(pool: DbPool) -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            pool,
        }
    }
}

impl Lobby {
    fn send_message(&self, room: &i64, message: &str, skip_id: Option<&i64>) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if let Some(sid) = skip_id {
                    if sid != id {
                        if let Some(addr) = self.sessions.get(id) {
                            addr.do_send(WsMessage(message.to_owned()));
                        }
                    }
                } 
            }
        }
    }

    fn send_unique_mes(&self, to: &i64, message: &str) {
        if let Some(addr) = self.sessions.get(to) {
            addr.do_send(WsMessage(message.to_owned()));
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // create a room if necessary, and then add the id to it
        self.rooms
            .entry(msg.lobby_id)
            .or_insert_with(HashSet::new).insert(msg.user_id);

        // send to everyone in the room that new user_id just joined
        self.send_message(&msg.lobby_id, format!("new user is connected: {}",msg.user_id).as_str(), Some(&msg.user_id));

        // store the address
        self.sessions.insert(
            msg.user_id,
            msg.addr,
        );

        // TODO: send to user old conversations
        // let pool_cloned = self.pool.clone();
        let mut conn = self.pool.get().unwrap();
        let mes = get_messages_for_room_id(&mut conn, &msg.lobby_id).unwrap();
        self.send_unique_mes(&msg.user_id, serde_json::to_string(&mes).unwrap().as_str());
    }
}


impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        // remove the session
        if self.sessions.remove(&msg.user_id).is_some() {
            
            // send message to everyone in the room that the user_id just left
            self.send_message(&msg.room_id, format!("user is disconnected: {}",msg.user_id).as_str(), Some(&msg.user_id));
            
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.user_id);
                } else {
                    //only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _: &mut Context<Self>) -> Self::Result {
        let dt = new_naive_date();

        let mes = InsertMessage {
            room_id: msg.room_id,
            sender_id: msg.user_id,
            msg: msg.msg,
            created_at: dt,
            updated_at: dt,
        };
        let res = self.send_message(&msg.room_id, &serde_json::to_string(&mes).unwrap(), Some(&msg.user_id));
        if let Ok(mut conn)= self.pool.get() {
            // TODO: make it async or use channel
            create_new_message_from_insert_struct(&mut conn, mes);
        }
        return res;
    }
}