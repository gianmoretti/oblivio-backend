#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20231103_114510_notes;

mod m20240331_173741_designateds;
mod m20240331_174528_assets;
mod m20240331_175235_asset_documents;
mod m20240402_235025_asset_designateds;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20231103_114510_notes::Migration),
            Box::new(m20240331_173741_designateds::Migration),
            Box::new(m20240331_174528_assets::Migration),
            Box::new(m20240331_175235_asset_documents::Migration),
            Box::new(m20240402_235025_asset_designateds::Migration),
        ]
    }
}