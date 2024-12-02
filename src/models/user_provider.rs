use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user_providers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserProvider {
    id: Option<i32>,
    user_id: i32,
    provider: String,
    provider_user_id: String,
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_at: Option<NaiveDateTime>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

