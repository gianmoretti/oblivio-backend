use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Designateds::Table)
                    .col(pk_auto(Designateds::Id))
                    .col(string(Designateds::Email))
                    .col(string(Designateds::FirstName))
                    .col(string(Designateds::LastName))
                    .col(date_null(Designateds::BirthDate))
                    .col(string_null(Designateds::BirthPlace))
                    .col(string_null(Designateds::Residence))
                    .col(string_null(Designateds::PhoneNumber))
                    .col(string_null(Designateds::FiscalCode))
                    .col(string_null(Designateds::Color))
                    .col(string_null(Designateds::ImageUrl))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Designateds::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Designateds {
    Table,
    Id,
    Email,
    FirstName,
    LastName,
    BirthDate,
    BirthPlace,
    Residence,
    PhoneNumber,
    FiscalCode,
    Color,
    ImageUrl,
    
}


