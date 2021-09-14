// use rocket::http::Status;
// use rocket::request::{FromRequest, Outcome};
// use rocket::{Request, State};
// use std::ops::Deref;
//use rocket::outcome::try_outcome;
//
use rocket::{Build, Rocket};
use rocket_sync_db_pools::{database, diesel};

#[database("mugendb")]
pub struct DbPool(diesel::PgConnection);

pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run` that runs the migrations in the
    // specified directory, initializing the database.
    embed_migrations!("../migrations");

    let conn = DbPool::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
}

/*
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<DbConn, Self::Error> {
        let pool = try_outcome!(request.guard::<>().await);
        //let pool = request.guard::<State<Pool>>().await.unwrap();
        match pool {
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
*/
