use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sentence::Table)
                    .if_not_exists()
                    .col(pk_auto(Sentence::Id))
                    .col(string(Sentence::Text))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(Sentence::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Sentence {
    Table,
    Id,
    Text,
}
