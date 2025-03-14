use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const UQ_PROFILES_NAME: &str = "UQ_profiles_name";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Profiles::Table)
                    .if_not_exists()
                    .col(pk_auto(Profiles::Id))
                    .col(string(Profiles::Name).not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Profiles::Table)
                    .col(Profiles::Name)
                    .unique()
                    .name(UQ_PROFILES_NAME)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .table(Profiles::Table)
                    .name(UQ_PROFILES_NAME)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Profiles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Profiles {
    Table,
    Id,
    Name,
}
