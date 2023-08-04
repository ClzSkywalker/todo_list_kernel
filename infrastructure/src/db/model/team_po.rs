use super::preclude::*;
use chrono::{DateTime, Local};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "team")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
    pub uid: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    UserTeam,
    Classify,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(UserEntity)
                .from(Column::Uid)
                .to(UserColumn::Id)
                .into(),
            Self::UserTeam => Entity::has_many(UserTeamEntity)
                .from(Column::Uid)
                .to(UserTeamColumn::Uid)
                .into(),
            Self::Classify => Entity::has_many(super::classify_po::Entity).into(),
        }
    }
}

impl Related<UserTeamEntity> for Entity {
    fn to() -> RelationDef {
        Relation::UserTeam.def()
    }
}

impl Related<UserEntity> for Entity {
    fn to() -> RelationDef {
        UserTeamRelation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(UserTeamRelation::Team.def().rev())
    }
}

impl Related<super::classify_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Classify.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
