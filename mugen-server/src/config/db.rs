use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn get_db_pool(
    host: String,
    port: String,
    database: String,
    user: String,
    password: String,
) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(format!(
        "postgresql://{}:{}@{}:{}/{}",
        user, password, host, port, database
    ));
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
