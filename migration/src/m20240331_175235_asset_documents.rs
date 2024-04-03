use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(AssetDocuments::Table)
                    .col(pk_auto(AssetDocuments::Id))
                    .col(string(AssetDocuments::MimeType))
                    .col(string(AssetDocuments::Name))
                    .col(string(AssetDocuments::Filename))
                    .col(string_null(AssetDocuments::Url))
                    .col(integer(AssetDocuments::AssetId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-asset_documents-assets")
                            .from(AssetDocuments::Table, AssetDocuments::AssetId)
                            .to(Assets::Table, Assets::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AssetDocuments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AssetDocuments {
    Table,
    Id,
    MimeType,
    Name,
    Filename,
    Url,
    AssetId,
    
}


#[derive(DeriveIden)]
enum Assets {
    Table,
    Id,
}
