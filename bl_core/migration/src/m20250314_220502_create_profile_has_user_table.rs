use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20220101_000001_create_users_table::Users, m20250314_220417_create_profiles_table::Profiles,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

const FK_PROFILE_HAS_USER_PROFILE_ID: &str = "FK_profileHasUser_profileId";
const FK_PROFILE_HAS_USER_USER_ID: &str = "FK_profileHasUser_userId";
const UQ_PROFILE_HAS_USER_PROFILE_ID_USER_ID: &str = "UQ_profileHasUser_profileId_userId";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProfileHasUser::Table)
                    .if_not_exists()
                    .col(integer(ProfileHasUser::ProfileId))
                    .col(integer(ProfileHasUser::UserId))
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from_col(ProfileHasUser::ProfileId)
                    .to_col(Profiles::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .name(FK_PROFILE_HAS_USER_PROFILE_ID)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from_col(ProfileHasUser::UserId)
                    .to_col(Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .name(FK_PROFILE_HAS_USER_USER_ID)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(ProfileHasUser::Table)
                    .col(ProfileHasUser::ProfileId)
                    .col(ProfileHasUser::UserId)
                    .unique()
                    .name(UQ_PROFILE_HAS_USER_PROFILE_ID_USER_ID)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .table(ProfileHasUser::Table)
                    .name(UQ_PROFILE_HAS_USER_PROFILE_ID_USER_ID)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(FK_PROFILE_HAS_USER_PROFILE_ID)
                    .table(ProfileHasUser::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(FK_PROFILE_HAS_USER_USER_ID)
                    .table(ProfileHasUser::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(ProfileHasUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProfileHasUser {
    Table,
    ProfileId,
    UserId,
}
