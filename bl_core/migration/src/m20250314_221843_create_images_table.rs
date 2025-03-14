use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const UQ_IMAGES_PATH: &str = "uq_images_path";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Images::Table)
                    .if_not_exists()
                    .col(pk_auto(Images::Id))
                    .col(string(Images::Path).not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name(UQ_IMAGES_PATH)
                    .table(Images::Table)
                    .col(Images::Path)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name(UQ_IMAGES_PATH)
                    .table(Images::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Images::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Images {
    Table,
    Id,
    Path,
}
