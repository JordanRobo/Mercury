// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Text,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
        bio -> Nullable<Text>,
        profile_picture -> Nullable<Text>,
    }
}

diesel::table! {
    post_tags (post_id, tag_id) {
        post_id -> Nullable<Text>,
        tag_id -> Nullable<Text>,
    }
}

diesel::table! {
    posts (id) {
        id -> Text,
        title -> Nullable<Text>,
        slug -> Nullable<Text>,
        content -> Nullable<Text>,
        feature_image -> Nullable<Text>,
        excerpt -> Nullable<Text>,
        published -> Nullable<Integer>,
        author_id -> Nullable<Text>,
    }
}

diesel::table! {
    tags (id) {
        id -> Text,
        name -> Nullable<Text>,
        slug -> Nullable<Text>,
    }
}

diesel::joinable!(post_tags -> posts (post_id));
diesel::joinable!(post_tags -> tags (tag_id));
diesel::joinable!(posts -> authors (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    post_tags,
    posts,
    tags,
);
