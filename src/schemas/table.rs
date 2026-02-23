// @generated automatically by Diesel CLI.

diesel::table! {
    attachments (id) {
        id -> Integer,
        user_id -> Text,
        filename -> Text,
        path -> Text,
        mime_type -> Text,
        size -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    refresh_tokens (id) {
        id -> Text,
        user_id -> Text,
        token -> Text,
        expires_at -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        email -> Text,
        username -> Text,
        password -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(refresh_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(attachments, refresh_tokens, users,);
