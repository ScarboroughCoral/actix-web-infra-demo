// @generated automatically by Diesel CLI.

diesel::table! {
    user_providers (id) {
        id -> Nullable<BigInt>,
        user_id -> BigInt,
        provider -> Text,
        provider_user_id -> Text,
        access_token -> Nullable<Text>,
        refresh_token -> Nullable<Text>,
        expires_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<BigInt>,
        email -> Nullable<Text>,
        username -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(user_providers -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    user_providers,
    users,
);
