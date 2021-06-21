use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_db_pool(url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}
