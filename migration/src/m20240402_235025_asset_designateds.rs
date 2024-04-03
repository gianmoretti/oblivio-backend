use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(AssetDesignateds::Table)
                    .col(pk_auto(AssetDesignateds::Id))
                    .col(integer(AssetDesignateds::AssetId))
                    .col(integer(AssetDesignateds::DesignatedId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-asset_designateds-assets")
                            .from(AssetDesignateds::Table, AssetDesignateds::AssetId)
                            .to(Assets::Table, Assets::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-asset_designateds-designateds")
                            .from(AssetDesignateds::Table, AssetDesignateds::DesignatedId)
                            .to(Designateds::Table, Designateds::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AssetDesignateds::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AssetDesignateds {
    Table,
    Id,
    AssetId,
    DesignatedId,
    
}


#[derive(DeriveIden)]
enum Assets {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Designateds {
    Table,
    Id,
}
