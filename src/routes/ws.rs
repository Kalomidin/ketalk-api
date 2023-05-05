use actix_web::{get, post, HttpMessage, Error, HttpResponse, HttpRequest};
use actix_web::web::{Path, Payload, Data, block, Json};
use actix::Addr;
use actix_web_actors::ws;
use diesel::result;
use diesel::result::Error as DieselError;

use crate::ws::lobby::Lobby;
use crate::ws::ws::WsConn;
use crate::repository::room::{get_room_by_id, get_room_by_name_and_creator, create_new_room};
use crate::repository::room_member::{create_new_room_member};
use super::DbPool;
use super::models::{CreateRoomRequest, CreateRoomResponse};
use super::RouteError;

#[get("/room/join/{room_id}")]
pub async fn join_room(
    req: HttpRequest,
    stream: Payload,
    room_id: Path<i64>,
    pool: Data<DbPool>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    println!("it is coming to here inside the room");
    // TODO: validate conversation id exists in db
    let ext = req.extensions();
    let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
    let rid = room_id.into_inner();
    let pool_cloned = pool.clone();
    block(move || {
        if let Ok(mut conn) = pool_cloned.get() {
            let res = get_room_by_id(&mut conn, &rid)?;
            return Ok(res);
        }
        return Err(RouteError::PoolingErr);
      })
      .await?
      .map_err(actix_web::error::ErrorUnprocessableEntity)?;
    println!("received new conn: {}", rid);
    let ws = WsConn::new(
        user_id,
        rid,
        srv.get_ref().clone(),
    );
    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
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