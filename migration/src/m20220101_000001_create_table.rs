use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
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
                    .col(ColumnDef::new(User::Uuid).string().unique_key().not_null())
                    .col(ColumnDef::new(User::TeamIdPort).string().not_null())
                    .col(ColumnDef::new(User::NickName).string())
                    .col(ColumnDef::new(User::MemberType).integer())
                    .col(ColumnDef::new(User::RegisterType).integer())
                    .col(ColumnDef::new(User::Picture).string())
                    .col(ColumnDef::new(User::Email).string())
                    .col(ColumnDef::new(User::Phone).string())
                    .col(ColumnDef::new(User::Pwd).string())
                    .col(ColumnDef::new(User::Version).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Team::Table)
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
                    .col(ColumnDef::new(Team::Uuid).string().unique_key().not_null())
                    .col(ColumnDef::new(Team::CreatedBy).string())
                    .col(ColumnDef::new(Team::Name).string())
                    .col(ColumnDef::new(Team::Description).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserToTeam::Table)
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
                    .col(ColumnDef::new(UserToTeam::Uid).string())
                    .col(ColumnDef::new(UserToTeam::Tid).string())
                    .col(ColumnDef::new(UserToTeam::Sort).integer())
                    .index(
                        Index::create()
                            .unique()
                            .name("udx_utt_uid_tid")
                            .col(UserToTeam::Uid)
                            .col(UserToTeam::Tid),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Classify::Table)
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
                    .col(ColumnDef::new(Classify::Uuid).string().unique_key())
                    .col(ColumnDef::new(Classify::CreatedBy).string())
                    .col(ColumnDef::new(Classify::TeamId).string())
                    .col(ColumnDef::new(Classify::Title).string())
                    .col(ColumnDef::new(Classify::Color).string())
                    .col(ColumnDef::new(Classify::ShowType).integer())
                    .col(ColumnDef::new(Classify::OrderType).integer())
                    .col(ColumnDef::new(Classify::Sort).integer())
                    .col(ColumnDef::new(Classify::ParentId).integer())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Devide::Table)
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
                    .col(ColumnDef::new(Devide::Uuid).string().unique_key())
                    .col(ColumnDef::new(Devide::ClassifyId).string())
                    .col(ColumnDef::new(Devide::CreatedBy).string())
                    .col(ColumnDef::new(Devide::Title).string())
                    .col(ColumnDef::new(Devide::Sort).integer())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TaskContent::Table)
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
                    .col(ColumnDef::new(TaskContent::Uuid).string().unique_key())
                    .col(ColumnDef::new(TaskContent::Content).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
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
                    .col(ColumnDef::new(Task::Uuid).string().unique_key())
                    .col(ColumnDef::new(Task::CreatedBy).integer())
                    .col(ColumnDef::new(Task::DevideId).integer())
                    .col(ColumnDef::new(Task::ContentId).integer())
                    .col(ColumnDef::new(Task::TaskModeId).integer())
                    .col(ColumnDef::new(Task::Title).string())
                    .col(ColumnDef::new(Task::CompletedAt).string())
                    .col(ColumnDef::new(Task::GiveUpAt).string())
                    .col(ColumnDef::new(Task::StartAt).string())
                    .col(ColumnDef::new(Task::EndAt).string())
                    .col(ColumnDef::new(Task::ParentId).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TaskMode::Table)
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
                    .col(ColumnDef::new(TaskMode::Uuid).string().unique_key())
                    .col(ColumnDef::new(TaskMode::CreatedBy).string().unique_key())
                    .col(ColumnDef::new(TaskMode::TeamId).integer())
                    .col(ColumnDef::new(TaskMode::ModeType).integer())
                    .col(ColumnDef::new(TaskMode::Config).json())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Team::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserToTeam::Table).to_owned())
            .await?;

        Ok(())
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
enum User {
    Table,
    Uuid,
    TeamIdPort,
    NickName,
    MemberType,
    RegisterType,
    Picture,
    Email,
    Phone,
    Pwd,
    Version,
}

#[derive(Iden)]
enum Team {
    Table,
    Uuid,
    CreatedBy,
    Name,
    Description,
}

#[derive(Iden)]
enum UserToTeam {
    Table,
    Uid,
    Tid,
    Sort,
}

#[derive(Iden)]
enum Classify {
    Table,
    Uuid,
    CreatedBy,
    TeamId,
    Title,
    Color,
    ShowType,
    OrderType,
    Sort,
    ParentId,
}

#[derive(Iden)]
enum Devide {
    Table,
    Uuid,
    ClassifyId,
    CreatedBy,
    Title,
    Sort,
}

#[derive(Iden)]
enum TaskContent {
    Table,
    Uuid,
    Content,
}

#[derive(Iden)]
enum Task {
    Table,
    Uuid,
    CreatedBy,
    DevideId,
    ContentId,
    TaskModeId,
    Title,
    CompletedAt,
    GiveUpAt,
    StartAt,
    EndAt,
    ParentId,
}

#[derive(Iden)]
enum TaskMode {
    Table,
    Uuid,
    CreatedBy,
    TeamId,
    ModeType,
    Config,
}
