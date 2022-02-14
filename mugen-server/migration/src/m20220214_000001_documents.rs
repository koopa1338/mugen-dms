use entity::document::*;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220214_000001_documents"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Column::Created).timestamp_with_time_zone())
                    .col(ColumnDef::new(Column::LastUpdated).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Column::Filetype)
                            .string()
                            .default(String::from("unknown")),
                    )
                    .col(
                        ColumnDef::new(Column::Version)
                            .integer()
                            .not_null()
                            .default(1i32),
                    )
                    .col(
                        ColumnDef::new(Column::Size)
                            .big_integer()
                            .not_null()
                            .default(0i64),
                    )
                    .col(ColumnDef::new(Column::Data).binary())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await
    }
}
