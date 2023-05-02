// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int8,
        user_name -> Varchar,
        phone_number -> Varchar,
    }
}
