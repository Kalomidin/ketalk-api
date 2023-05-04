// @generated automatically by Diesel CLI.

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
    users (id) {
        id -> Int8,
        user_name -> Varchar,
        phone_number -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(refresh_token -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    refresh_token,
    users,
);
