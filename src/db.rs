use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{SqliteConnection};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
