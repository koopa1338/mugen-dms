use entity::prelude::Documents;
pub use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Docs {
    Id,
    Created,
    Updated,
    Filetype,
    Version,
    Size,
    Data,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Documents)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Docs::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Docs::Created)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Docs::Updated).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Docs::Filetype)
                            .string()
                            .default(String::from("unknown")),
                    )
                    .col(
                        ColumnDef::new(Docs::Version)
                            .integer()
                            .not_null()
                            .default(1i32),
                    )
                    .col(
                        ColumnDef::new(Docs::Size)
                            .big_integer()
                            .not_null()
                            .default(0i64),
                    )
                    .col(ColumnDef::new(Docs::Data).binary())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Documents).to_owned())
            .await
    }
}
