// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Text,
        name -> Text,
        slug -> Text,
        email -> Text,
        bio -> Nullable<Text>,
        profile_picture -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    enquiries (id) {
        id -> Text,
        name -> Text,
        surname -> Nullable<Text>,
        email -> Text,
        job_title -> Nullable<Text>,
        company -> Nullable<Text>,
        enquiry_type -> Nullable<Text>,
        subject -> Nullable<Text>,
        message -> Text,
        status -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    media (file_path) {
        id -> Text,
        file_name -> Text,
        file_type -> Text,
        file_size -> Integer,
        file_path -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
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
        title -> Text,
        slug -> Text,
        excerpt -> Nullable<Text>,
        content -> Nullable<Text>,
        author_id -> Nullable<Text>,
        feature_image -> Nullable<Text>,
        status -> Nullable<Text>,
        published_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    subscriptions (id) {
        id -> Nullable<Text>,
        email -> Text,
        status -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tags (id) {
        id -> Text,
        name -> Text,
        slug -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        api_key -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(authors -> media (profile_picture));
diesel::joinable!(post_tags -> posts (post_id));
diesel::joinable!(post_tags -> tags (tag_id));
diesel::joinable!(posts -> authors (author_id));
diesel::joinable!(posts -> media (feature_image));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    enquiries,
    media,
    post_tags,
    posts,
    subscriptions,
    tags,
    users,
);
