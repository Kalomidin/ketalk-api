use actix::ActorFutureExt;
use actix::{fut, ActorContext};
use actix::{Actor, Addr, ContextFutureSpawner, Running, StreamHandler, WrapFuture};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use std::time::{Duration, Instant};

use super::lobby::Lobby;
use super::messages::{
  ClientActorMessage, ClientWsMessage, ClientWsMessageType, Connect, Disconnect, WsMessage,
};

// TODO: move to env
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsConn {
  user_id: i64,
  hb: Instant,
  room: i64,
  user_name: String,
  lobby_addr: Addr<Lobby>,
}

impl WsConn {
  pub fn new(user_id: i64, room: i64, user_name: String, lobby_addr: Addr<Lobby>) -> Self {
    Self {
      user_id,
      hb: Instant::now(),
      room,
      user_name,
      lobby_addr,
    }
  }
}

impl Actor for WsConn {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    self.hb(ctx);

    let addr = ctx.address();
    self
      .lobby_addr
      .send(Connect {
        addr: addr.recipient(),
        lobby_id: self.room,
        user_id: self.user_id,
      })
      .into_actor(self)
      .then(|res, _, ctx| {
        match res {
          Ok(_res) => (),
          _ => ctx.stop(),
        }
        fut::ready(())
      })
      .wait(ctx);
  }

  fn stopping(&mut self, _: &mut Self::Context) -> Running {
    self.lobby_addr.do_send(Disconnect {
      user_id: self.user_id,
      room_id: self.room,
    });
    Running::Stop
  }
}

impl Handler<WsMessage> for WsConn {
  type Result = ();
  fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
    ctx.text(msg.0);
  }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => {
        self.hb = Instant::now();
        ctx.pong(&msg);
      }
      Ok(ws::Message::Pong(_)) => {
        self.hb = Instant::now();
      }
      Ok(ws::Message::Binary(bin)) => {
        ctx.binary(bin);
      }
      Ok(ws::Message::Close(reason)) => {
        ctx.close(reason);
        ctx.stop();
        self.lobby_addr.do_send(ClientActorMessage {
          user_id: self.user_id,
          user_name: self.user_name.clone(),
          msg: ClientWsMessage {
            message_type: ClientWsMessageType::Closed,
            message: "".to_string(),
          },
          room_id: self.room,
        })
      }
      Ok(ws::Message::Continuation(_)) => {
        ctx.stop();
      }
      Ok(ws::Message::Nop) => {}
      Ok(Text(s)) => match serde_json::from_str::<ClientWsMessage>(&s) {
        Ok(m) => self.lobby_addr.do_send(ClientActorMessage {
          user_id: self.user_id,
          user_name: self.user_name.clone(),
          msg: m,
          room_id: self.room,
        }),
        _ => {
          println!("error parsing message");
          ctx.stop();
          return;
        }
      },
      Err(e) => {
        println!("error handling message: {:?}", e);
        // TODO: handle the error properly
        ctx.stop();
        return;
      }
    }
  }
}

impl WsConn {
  fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
    ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
      if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
        println!("Disconnecting failed heartbeat");
        act.lobby_addr.do_send(Disconnect {
          user_id: act.user_id,
          room_id: act.room,
        });
        ctx.stop();
        return;
      }

      ctx.ping(b"PING");
    });
  }
}
