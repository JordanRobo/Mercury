// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        #[max_length = 20]
        id -> Varchar,
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
        #[max_length = 20]
        post_id -> Varchar,
        #[max_length = 20]
        tag_id -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        #[max_length = 20]
        id -> Varchar,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        #[max_length = 255]
        slug -> Nullable<Varchar>,
        content -> Nullable<Text>,
        #[max_length = 255]
        feature_image -> Nullable<Varchar>,
        excerpt -> Nullable<Text>,
        published -> Nullable<Bool>,
        #[max_length = 20]
        author_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    tags (id) {
        #[max_length = 20]
        id -> Varchar,
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
