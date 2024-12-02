use chrono::NaiveDateTime;

pub struct UserProvider {
    id: Option<i64>,
    user_id: i64,
    provider: String,
    provider_user_id: String,
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_at: Option<NaiveDateTime>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}
