// @generated automatically by Diesel CLI.

diesel::table! {
    audit_logs (log_id) {
        log_id -> Text,
        user_id -> Text,
        activity_type -> Text,
        timestamp -> Timestamp,
        description -> Text,
    }
}

diesel::table! {
    authors (author_id) {
        author_id -> Text,
        user_id -> Text,
        slug -> Text,
        bio -> Nullable<Text>,
        profile_picture -> Nullable<Text>,
    }
}

diesel::table! {
    categories (category_id) {
        category_id -> Text,
        name -> Text,
        slug -> Text,
    }
}

diesel::table! {
    media (media_id) {
        media_id -> Text,
        file_name -> Text,
        file_type -> Text,
        file_size -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pass_reset_tokens (token_id) {
        token_id -> Text,
        user_id -> Text,
        token_value -> Text,
        token_expiry -> Timestamp,
    }
}

diesel::table! {
    post_categories (id) {
        id -> Text,
        post_id -> Text,
        category_id -> Text,
    }
}

diesel::table! {
    post_tags (id) {
        id -> Text,
        post_id -> Text,
        tag_id -> Text,
    }
}

diesel::table! {
    posts (post_id) {
        post_id -> Text,
        title -> Text,
        slug -> Text,
        excerpt -> Nullable<Text>,
        content -> Nullable<Text>,
        author_id -> Text,
        feature_image -> Nullable<Text>,
        status -> Nullable<Text>,
        published_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    related_posts (id) {
        id -> Text,
        post_id -> Text,
        related_post_id -> Text,
        strength -> Integer,
    }
}

diesel::table! {
    roles (role_id) {
        role_id -> Text,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    site_analytics (analytics_id) {
        analytics_id -> Text,
    }
}

diesel::table! {
    tags (tag_id) {
        tag_id -> Text,
        name -> Text,
        slug -> Text,
    }
}

diesel::table! {
    user_roles (id) {
        id -> Text,
        user_id -> Text,
        role_id -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Text,
        email -> Text,
        first_name -> Text,
        last_name -> Nullable<Text>,
        pass_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_login -> Timestamp,
    }
}

diesel::joinable!(audit_logs -> users (user_id));
diesel::joinable!(authors -> media (profile_picture));
diesel::joinable!(authors -> users (user_id));
diesel::joinable!(post_categories -> categories (category_id));
diesel::joinable!(post_categories -> posts (post_id));
diesel::joinable!(post_tags -> posts (post_id));
diesel::joinable!(post_tags -> tags (tag_id));
diesel::joinable!(posts -> authors (author_id));
diesel::joinable!(posts -> media (feature_image));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    audit_logs,
    authors,
    categories,
    media,
    pass_reset_tokens,
    post_categories,
    post_tags,
    posts,
    related_posts,
    roles,
    site_analytics,
    tags,
    user_roles,
    users,
);
