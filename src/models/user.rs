use chrono::NaiveDateTime;
use diesel::{ Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    id: Option<i32>,
    email: Option<String>,
    username: Option<String>,
    avatar_url: Option<String>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}
