// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Text,
    }
}
