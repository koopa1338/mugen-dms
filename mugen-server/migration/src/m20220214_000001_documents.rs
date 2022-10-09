pub use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub(crate) enum Document {
    Table,
    Id,
    Created,
    Updated,
    Filetype,
    Version,
    Size,
    Data,
    CategoryId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Document::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Document::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Document::Created)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Document::Updated).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Document::Filetype)
                            .string()
                            .default(String::from("unknown")),
                    )
                    .col(
                        ColumnDef::new(Document::Version)
                            .integer()
                            .not_null()
                            .default(1i32),
                    )
                    .col(
                        ColumnDef::new(Document::Size)
                            .big_integer()
                            .not_null()
                            .default(0i64),
                    )
                    .col(ColumnDef::new(Document::Data).binary())
                    .clone(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Document::Table).clone())
            .await
    }
}
