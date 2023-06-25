use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

use crate::schema::geofence as geofence_table;
use crate::schema::geofence::dsl::*;

#[derive(Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = geofence_table)]
pub struct InsertGeofence {
  pub name: String,
  pub geofence_type: String,
  pub parent_region_id: i64,
}

#[derive(Clone, Serialize, Deserialize, Queryable)]
pub struct Geofence {
  pub id: i64,
  pub name: String,
  pub geofence_type: String,
  pub parent_region_id: i64,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
  pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub fn add_geofence(
  conn: &mut PgConnection,
  _name: String,
  _geofence_type: String,
  _parent_region_id: i64,
) -> Result<Geofence, DieselError> {
  let new_geo = InsertGeofence {
    name: _name,
    geofence_type: _geofence_type,
    parent_region_id: _parent_region_id,
  };
  let resp = diesel::insert_into(geofence)
    .values(&new_geo)
    .get_result::<Geofence>(conn)?;
  return Ok(resp);
}

pub fn get_geofences(conn: &mut PgConnection) -> Result<Vec<Geofence>, DieselError> {
  let result = geofence.load(conn).optional()?;
  match result {
    Some(geofences) => Ok(geofences),
    None => Ok(vec![]),
  }
}
