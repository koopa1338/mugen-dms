pub use sea_schema::migration::*;

mod m20220101_000001_documents;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_documents::Migration)]
    }
}
