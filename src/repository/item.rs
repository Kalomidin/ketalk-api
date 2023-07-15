use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

use crate::schema::item as item_table;
use crate::schema::item::dsl::*;
use crate::schema::purchase as purchase_table;
use crate::schema::purchase::dsl::{
  buyer_id as purchase_buyer_id, item_id as purchase_item_id, purchase,
};
use crate::schema::user_favorite::dsl::{
  item_id as user_favorite_item_id, user_favorite, user_id as user_favorite_user_id,
};
use crate::schema::user_favorite::is_favorite;

#[derive(Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = item_table)]
pub struct InsertItem {
  pub title: String,
  pub description: String,
  pub price: i64,
  pub negotiable: bool,
  pub owner_id: i64,
  pub size: f64,
  pub karat_id: i64,
  pub weight: f64,
  pub category_id: i64,
  pub geofence_id: i64,
}

#[derive(Clone, Serialize, Deserialize, Queryable)]
pub struct Item {
  pub id: i64,
  pub title: String,
  pub description: String,
  pub price: i64,
  pub negotiable: bool,
  pub owner_id: i64,
  pub item_status: String,
  pub is_hidden: bool,
  pub favorite_count: i32,
  pub message_count: i32,
  pub seen_count: i32,
  pub size: f64,
  pub weight: f64,
  pub karat_id: i64,
  pub category_id: i64,
  pub geofence_id: i64,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Clone, Serialize, Deserialize, Queryable)]
pub struct Purchase {
  pub id: i64,
  pub buyer_id: i64,
  pub seller_id: i64,
  pub item_id: i64,
  pub created_at: NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = purchase_table)]
pub struct CreatePurchase {
  pub buyer_id: i64,
  pub seller_id: i64,
  pub item_id: i64,
}

pub fn insert_new_item(
  conn: &mut PgConnection,
  _owner_id: i64,
  _title: String,
  _description: String,
  _price: i64,
  _negotiable: bool,
  _size: f64,
  _weight: f64,
  _karat_id: i64,
  _category_id: i64,
  _geofence_id: i64,
) -> Result<Item, DieselError> {
  let new_item = InsertItem {
    owner_id: _owner_id,
    title: _title,
    description: _description,
    negotiable: _negotiable,
    price: _price,
    weight: _weight,
    size: _size,
    karat_id: _karat_id,
    category_id: _category_id,
    geofence_id: _geofence_id,
  };

  let resp = diesel::insert_into(item)
    .values(&new_item)
    .get_result(conn)?;
  return Ok(resp);
}

pub fn get_items_by_user_id(
  conn: &mut PgConnection,
  user_id: i64,
) -> Result<Vec<Item>, DieselError> {
  let result = item.filter(owner_id.eq(user_id)).load(conn).optional()?;
  match result {
    Some(val) => Ok(val),
    None => Err(DieselError::NotFound),
  }
}

pub fn update_item_status(
  conn: &mut PgConnection,
  item_id: i64,
  user_id: i64,
  new_item_status: String,
) -> Result<(), DieselError> {
  let result = diesel::update(item)
    .filter(id.eq(item_id).and(owner_id.eq(user_id)))
    .set(item_status.eq(new_item_status))
    .execute(conn);
  if result.is_err() || result.unwrap() == 0 {
    return Err(DieselError::NotFound);
  }
  Ok(())
}

pub fn get_item_by_id(conn: &mut PgConnection, item_id: i64) -> Result<Item, DieselError> {
  let result = item
    .filter(id.eq(item_id).and(deleted_at.is_null()))
    .first(conn)
    .optional()?;
  match result {
    Some(val) => Ok(val),
    None => Err(DieselError::NotFound),
  }
}

pub fn get_all_visible(conn: &mut PgConnection) -> Result<Vec<Item>, DieselError> {
  let result = item
    .filter(deleted_at.is_null().and(is_hideen.eq(false)))
    .order(created_at.desc())
    .load(conn)
    .optional()?;
  match result {
    Some(val) => Ok(val),
    None => Err(DieselError::NotFound),
  }
}

pub fn increment_message_count(conn: &mut PgConnection, item_id: i64) -> Result<(), DieselError> {
  let result = diesel::update(item)
    .filter(deleted_at.is_null().and(id.eq(item_id)))
    .set(message_count.eq(message_count + 1))
    .execute(conn);
  if result.is_err() || result.unwrap() == 0 {
    return Err(DieselError::NotFound);
  }
  Ok(())
}

pub fn update_favorite_count(
  conn: &mut PgConnection,
  item_id: i64,
  count: i32,
) -> Result<(), DieselError> {
  let result = diesel::update(item)
    .filter(deleted_at.is_null().and(id.eq(item_id)))
    .set(favorite_count.eq(favorite_count + count))
    .execute(conn);
  if result.is_err() || result.unwrap() == 0 {
    return Err(DieselError::NotFound);
  }
  Ok(())
}

pub fn hide_unhide_item(
  conn: &mut PgConnection,
  item_id: i64,
  _is_hidden: bool,
) -> Result<(), DieselError> {
  let result = diesel::update(item)
    .filter(deleted_at.is_null().and(id.eq(item_id)))
    .set(is_hideen.eq(_is_hidden))
    .execute(conn);
  if result.is_err() || result.unwrap() == 0 {
    return Err(DieselError::NotFound);
  }
  Ok(())
}

pub fn get_favorite_items(
  conn: &mut PgConnection,
  _user_id: i64,
) -> Result<Vec<Item>, DieselError> {
  let result = item
    .inner_join(user_favorite)
    .select(item::all_columns())
    .filter(
      user_favorite_item_id
        .eq(id)
        .and(user_favorite_user_id.eq(_user_id))
        .and(is_favorite.eq(true))
        .and(deleted_at.is_null()),
    )
    .load(conn)
    .optional()?;
  match result {
    Some(val) => Ok(val),
    None => Err(DieselError::NotFound),
  }
}

pub fn create_purchase(
  conn: &mut PgConnection,
  _buyer_id: i64,
  _seller_id: i64,
  _item_id: i64,
) -> Result<Purchase, DieselError> {
  let new_purchase = CreatePurchase {
    buyer_id: _buyer_id,
    seller_id: _seller_id,
    item_id: _item_id,
  };

  let resp = diesel::insert_into(purchase)
    .values(&new_purchase)
    .get_result(conn)?;
  return Ok(resp);
}

pub fn get_purchase_for_item(
  conn: &mut PgConnection,
  _item_id: i64,
) -> Result<Purchase, DieselError> {
  let result = purchase.filter(purchase_item_id.eq(_item_id)).first(conn).optional()?;
  match result {
    Some(val) => Ok(val),
    None => Err(DieselError::NotFound),
  }
}

pub fn get_purchased_items(
  conn: &mut PgConnection,
  buyer_id: i64,
) -> Result<Vec<Item>, DieselError> {
  let result = item
    .inner_join(purchase)
    .select(item::all_columns())
    .filter(
      purchase_buyer_id
        .eq(buyer_id)
        .and(deleted_at.is_null())
        .and(id.eq(purchase_item_id)),
    )
    .load(conn)
    .optional()?;
  match result {
    Some(val) => Ok(val),
    None => Err(DieselError::NotFound),
  }
}
