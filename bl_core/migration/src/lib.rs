pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;
mod m20250314_213054_create_roles_table;
mod m20250314_213117_create_permissions_table;
mod m20250314_213216_create_user_has_role_table;
mod m20250314_213232_create_role_has_permission_table;
mod m20250314_220417_create_profiles_table;
mod m20250314_220502_create_profile_has_user_table;
mod m20250314_221843_create_images_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20250314_213054_create_roles_table::Migration),
            Box::new(m20250314_213117_create_permissions_table::Migration),
            Box::new(m20250314_213216_create_user_has_role_table::Migration),
            Box::new(m20250314_213232_create_role_has_permission_table::Migration),
            Box::new(m20250314_220417_create_profiles_table::Migration),
            Box::new(m20250314_220502_create_profile_has_user_table::Migration),
            Box::new(m20250314_221843_create_images_table::Migration),
        ]
    }
}
