// @generated automatically by Diesel CLI.

diesel::table! {
    item (id) {
        id -> Int8,
        description -> Varchar,
        details -> Varchar,
        price -> Int8,
        negotiable -> Bool,
        owner_id -> Int8,
        item_status -> Varchar,
        favorite_count -> Int4,
        message_count -> Int4,
        seen_count -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    item_document (id) {
        id -> Int8,
        key -> Varchar,
        item_id -> Int8,
        user_id -> Int8,
        is_cover -> Bool,
        uploaded_to_cloud -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    message (id) {
        id -> Int8,
        room_id -> Int8,
        sender_id -> Int8,
        sender_name -> Varchar,
        msg -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    refresh_token (id) {
        id -> Int8,
        user_id -> Int8,
        token -> Varchar,
        created_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    room (id) {
        id -> Int8,
        item_id -> Nullable<Int8>,
        created_by -> Int8,
        created_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    room_member (id) {
        id -> Int8,
        room_id -> Int8,
        member_id -> Int8,
        created_at -> Timestamptz,
        last_joined_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        user_name -> Varchar,
        phone_number -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(item -> users (owner_id));
diesel::joinable!(item_document -> item (item_id));
diesel::joinable!(item_document -> users (user_id));
diesel::joinable!(message -> room (room_id));
diesel::joinable!(message -> users (sender_id));
diesel::joinable!(refresh_token -> users (user_id));
diesel::joinable!(room -> item (item_id));
diesel::joinable!(room -> users (created_by));
diesel::joinable!(room_member -> room (room_id));
diesel::joinable!(room_member -> users (member_id));

diesel::allow_tables_to_appear_in_same_query!(
  item,
  item_document,
  message,
  refresh_token,
  room,
  room_member,
  users,
);
