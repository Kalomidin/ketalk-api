// @generated automatically by Diesel CLI.

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
        name -> Varchar,
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

diesel::joinable!(message -> room (room_id));
diesel::joinable!(message -> users (sender_id));
diesel::joinable!(refresh_token -> users (user_id));
diesel::joinable!(room -> users (created_by));
diesel::joinable!(room_member -> room (room_id));
diesel::joinable!(room_member -> users (member_id));

diesel::allow_tables_to_appear_in_same_query!(
    message,
    refresh_token,
    room,
    room_member,
    users,
);
