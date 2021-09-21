use rocket::{Build, Rocket};
use rocket_sync_db_pools::{database, diesel};

#[database("mugendb")]
pub struct DbPool(diesel::PgConnection);

pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // Get migrations
    embed_migrations!("../migrations");

    let conn = DbPool::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
}
