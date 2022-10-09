use sea_orm::DatabaseConnection;
pub use sea_orm_migration::prelude::*;

mod m20220214_000001_documents;
mod m20221009_130143_categories;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220214_000001_documents::Migration),
            Box::new(m20221009_130143_categories::Migration),
        ]
    }
}

/// # Errors
///
/// Will return `DbErr` on unsuccessful database operations.
pub async fn migrate_database(connection: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(connection, None).await
}
