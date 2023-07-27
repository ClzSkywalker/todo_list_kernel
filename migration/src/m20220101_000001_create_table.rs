use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Base::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Base::CreatedAt).date_time())
                    .col(ColumnDef::new(Base::UpdatedAt).date_time())
                    .col(ColumnDef::new(Base::DeletedAt).date_time())
                    .col(ColumnDef::new(File::Name).string().not_null())
                    .col(ColumnDef::new(File::FileType).string().not_null())
                    .col(ColumnDef::new(File::Size).integer())
                    .col(ColumnDef::new(File::Width).integer())
                    .col(ColumnDef::new(File::Height).integer())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(File::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Base {
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum File {
    Table,
    Name,
    Width,
    Height,
    Size,
    FileType,
}
