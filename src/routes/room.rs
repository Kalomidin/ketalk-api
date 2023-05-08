use actix_web::{get, post, HttpMessage, Error, HttpResponse, HttpRequest};
use actix_web::web::{Path, Payload, Data, block, Json};
use actix::Addr;
use actix_web_actors::ws;
use diesel::result;
use diesel::result::Error as DieselError;

use crate::repository::user::get_user_by_id;
use crate::ws::lobby::Lobby;
use crate::ws::ws::WsConn;
use crate::repository::room::{get_room_by_name_and_creator, create_new_room};
use crate::repository::room_member::{create_new_room_member, get_room_member, get_rooms_by_user_id};
use crate::repository::message::{get_last_message_by_room_id};
use super::DbPool;
use super::models::{CreateRoomRequest, CreateRoomResponse, GetUserRoomsResponse, UserRoom};
use super::RouteError;

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
        rid_string[0..rid_string.len()-1].parse::<i64>()
    } else {
        rid_string.parse::<i64>()
    };
    let rid = if let Ok(rid) = rid {
        rid
    } else {
        return Ok(HttpResponse::BadRequest().finish())
    };
    let pool_cloned = pool.clone();
    let user = block(move || {
        if let Ok(mut conn) = pool_cloned.get() {
            get_room_member(&mut conn, &user_id, &rid)?;
            let user = get_user_by_id(&mut conn, user_id)?;
            return Ok(user)
        }
        return Err(RouteError::PoolingErr);
      })
      .await?
      .map_err(actix_web::error::ErrorUnprocessableEntity)?;

    let ws = WsConn::new(
        user.id,
        rid,
        user.user_name,
        srv.get_ref().clone(),
    );
    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}

#[get("/room/getUserRooms")]
pub async fn get_user_rooms(
    pool: Data<DbPool>,
    req: HttpRequest,
) ->  Result<HttpResponse, Error> {
    let ext = req.extensions();
    let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
    println!("get_user_rooms: ${}", user_id);

    let pool_cloned = pool.clone();
    let resp = block(move || {
        if let Ok(mut conn) = pool_cloned.get() {
            let mut resp = GetUserRoomsResponse{
                rooms: Vec::<UserRoom>::new(),
            };
            let rooms = get_rooms_by_user_id(&mut conn, &user_id)?;
            // Get for each room last message
            for room in rooms {
                if let Ok(mes) = get_last_message_by_room_id(&mut conn, &room.room_id) {
                    resp.rooms.push(UserRoom{
                        room_name: room.room_name,
                        last_message: mes.msg,
                        last_message_time: mes.created_at.to_string(),
                        last_message_sender_id: mes.sender_id,
                        room_id: room.room_id,
                    });
                }
            }
            return Ok(resp)
        }
        return Err(RouteError::PoolingErr);        
      })
      .await?;
    match resp {
        Ok(rooms) => {
            Ok(HttpResponse::Ok().json(rooms))
        },
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
    println!("it is coming to here");
    let secondary_user_id = form.secondary_user_id.clone();
    let room_name = form.room_name.clone();
    let ext = req.extensions();
    let user_id: i64 = ext.get::<i64>().unwrap().to_owned();

    // Check if there existing room for the given room name, if not create new room
    let pool_cloned = pool.clone();
    let resp = block(move || {
        if let Ok(mut conn) = pool_cloned.get() {
            let res = get_room_by_name_and_creator(&mut conn , &user_id, &secondary_user_id, &room_name)?;
            return Ok(res);
        }
        return Err(RouteError::PoolingErr);        
      })
      .await?;
    match resp {
        Ok(room) => {
            println!("room already exists");
            Ok(HttpResponse::Ok().json(CreateRoomResponse{
                room_id: room.id,
                room_name: room.name,
                secondary_user_id: secondary_user_id,
            }))
        },
        Err(RouteError::DbError(DieselError::NotFound)) => {
            let pool_cloned = pool.clone();
            let room_name = form.room_name.clone();
            let resp = block(move || {
                if let Ok(mut conn) = pool_cloned.get() {
                    let room = create_new_room(&mut conn, &user_id, &room_name)?;
                    create_new_room_member(&mut conn, &room.id, &user_id)?;
                    create_new_room_member(&mut conn, &room.id, &secondary_user_id)?;
                    return Ok(room);
                }
                return Err(RouteError::PoolingErr);
            })
            .await?
            .map_err(actix_web::error::ErrorUnprocessableEntity)?;
            return Ok(HttpResponse::Ok().json(CreateRoomResponse {
                secondary_user_id: secondary_user_id,
                room_name: form.room_name.to_owned(),
                room_id: resp.id,
            }))
        },
        Err(e) => {
            return Err(actix_web::error::ErrorInternalServerError("Internal Server Error"));
        }
    }
}