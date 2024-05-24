// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Text,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        authorid -> Int4,
        published -> Nullable<Bool>,
    }
}


diesel::joinable!(posts -> authors (authorid));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    posts,
    
);
