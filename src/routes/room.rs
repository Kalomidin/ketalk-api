use actix::Addr;
use actix_web::web::{block, Data, Json, Path, Payload};
use actix_web::{get, post, Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use diesel::result::Error as DieselError;

use super::models::{CreateRoomRequest, CreateRoomResponse, GetUserRoomsResponse, UserRoom};
use super::DbPool;
use super::{route_error_handler, RouteError};
use crate::repository::item::{get_item_by_id, increment_message_count};
use crate::repository::item_image::get_cover_pic_for_item;
use crate::repository::message::get_last_message_by_room_id;
use crate::repository::room::{create_new_room, get_room_by_item_and_creator};
use crate::repository::room_member::{
  create_new_room_member, get_room_member, get_rooms_by_user_id, set_last_joined_at,
};
use crate::repository::user::get_user_by_id;
use crate::routes::item::CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME;
use crate::ws::lobby::Lobby;
use crate::ws::ws::WsConn;

#[get("/room/join/{room_id}")]
pub async fn join_room(
  req: HttpRequest,
  stream: Payload,
  room_id: Path<String>,
  pool: Data<DbPool>,
  srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
  println!("join_room: ${}", room_id);
  req.headers().iter().for_each(|(key, value)| {
    println!("{}: {}", key, value.to_str().unwrap());
  });
  // TODO: validate conversation id exists in db
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  let rid_string = room_id.into_inner().clone();
  let rid = if rid_string.contains('#') {
    rid_string[0..rid_string.len() - 1].parse::<i64>()
  } else {
    rid_string.parse::<i64>()
  };
  let rid = if let Ok(rid) = rid {
    rid
  } else {
    return Ok(HttpResponse::BadRequest().finish());
  };
  let pool_cloned = pool.clone();
  let user = block(move || {
    if let Ok(mut conn) = pool_cloned.get() {
      get_room_member(&mut conn, &user_id, &rid)?;
      set_last_joined_at(&mut conn, &user_id, &rid)?;
      let user = get_user_by_id(&mut conn, user_id)?;
      return Ok(user);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;

  let ws = WsConn::new(user.id, rid, user.name, srv.get_ref().clone());
  let resp = ws::start(ws, &req, stream)?;
  Ok(resp)
}

#[get("/room/getUserRooms")]
pub async fn get_user_rooms(pool: Data<DbPool>, req: HttpRequest) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  println!("get_user_rooms: ${}", user_id);

  let pool_cloned = pool.clone();
  let resp = block(move || {
    if let Ok(mut conn) = pool_cloned.get() {
      let mut resp = GetUserRoomsResponse {
        rooms: Vec::<UserRoom>::new(),
      };
      let rooms = get_rooms_by_user_id(&mut conn, &user_id)?;
      // Get for each room last message
      for room in rooms {
        if room.item_id == None {
          continue;
        }
        let item_id = room.item_id.unwrap();
        if let Ok(mes) = get_last_message_by_room_id(&mut conn, &room.room_id) {
          let item = get_item_by_id(&mut conn, item_id)?;
          let cover_image_doc = get_cover_pic_for_item(&mut conn, item.id)?;
          resp.rooms.push(UserRoom {
            description: item.description,
            item_image_url: format!(
              "https://{}/{}",
              CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME, cover_image_doc.key,
            ),
            secondary_user_image_url: format!(
              "https://{}/{}",
              CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME, cover_image_doc.key,
            ),
            item_id: item.id,
            last_message: mes.msg,
            last_message_time: mes.created_at,
            last_message_sender_id: mes.sender_id,
            room_id: room.room_id,
            is_message_read: mes.sender_id == user_id || mes.created_at <= room.last_joined_at,
          });
        }
      }
      return Ok(resp);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?;
  match resp {
    Ok(rooms) => Ok(HttpResponse::Ok().json(rooms)),
    Err(e) => {
      println!("error: {:?}", e);
      Ok(HttpResponse::InternalServerError().finish())
    }
  }
}

#[post("/room/createRoom")]
pub async fn create_room(
  pool: Data<DbPool>,
  req: HttpRequest,
  form: Json<CreateRoomRequest>,
) -> Result<HttpResponse, Error> {
  println!("creating new room");
  let secondary_user_id = form.secondary_user_id.clone();
  let item_id = form.item_id.clone();
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();

  // Check if there existing room for the given room name, if not create new room
  let pool_cloned = pool.clone();
  let resp = block(move || {
    if let Ok(mut conn) = pool_cloned.get() {
      match get_room_by_item_and_creator(&mut conn, &user_id, &secondary_user_id, &item_id) {
        Ok(res) => {
          return Ok(res);
        }
        Err(DieselError::NotFound) => {
          let room = create_new_room(&mut conn, &user_id, &item_id)?;

          // increment message count
          increment_message_count(&mut conn, item_id)?;

          create_new_room_member(&mut conn, &room.id, &user_id)?;
          create_new_room_member(&mut conn, &room.id, &secondary_user_id)?;
          return Ok(room);
        }
        Err(e) => {
          return Err(RouteError::DbError(e));
        }
      }
    }
    return Err(RouteError::PoolingErr);
  })
  .await?;
  match resp {
    Ok(room) => Ok(HttpResponse::Ok().json(CreateRoomResponse {
      room_id: room.id,
      item_id: item_id,
      secondary_user_id: secondary_user_id,
    })),
    Err(e) => {
      return Err(actix_web::error::ErrorInternalServerError(format!(
        "Internal Server Error: {:?}",
        e.to_string()
      )));
    }
  }
}
