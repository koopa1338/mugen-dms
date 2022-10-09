use sea_orm_migration::prelude::*;

use crate::m20220214_000001_documents::Document;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Category {
    Table,
    Id,
    Title,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Category::Title).string().not_null())
                    .clone(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Document::Table)
                    .add_column(ColumnDef::new(Document::CategoryId).integer())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("FK_document_category")
                            .from_tbl(Document::Table)
                            .from_col(Document::CategoryId)
                            .to_tbl(Category::Table)
                            .to_col(Category::Id),
                    )
                    .clone(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Document::Table)
                    .drop_column(Document::CategoryId)
                    .drop_foreign_key(Alias::new("FK_document_category"))
                    .clone(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Category::Table).clone())
            .await
    }
}
