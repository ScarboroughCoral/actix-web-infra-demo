use chrono::NaiveDateTime;

pub struct User {
    id: i64,
    email: Option<String>,
    username: String,
    avatar_url: Option<String>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}
