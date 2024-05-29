// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        #[max_length = 255]
        profile_picture -> Nullable<Varchar>,
    }
}

diesel::table! {
    post_tags (post_id, tag_id) {
        post_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        #[max_length = 255]
        slug -> Nullable<Varchar>,
        content -> Nullable<Text>,
        #[max_length = 255]
        feature_image -> Nullable<Varchar>,
        excerpt -> Nullable<Text>,
        published -> Nullable<Bool>,
        author_id -> Nullable<Int4>,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        slug -> Nullable<Varchar>,
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
