use super::preclude::*;
use common::DateTimeLocal;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "team")]
#[sea_query::enum_def]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: Option<DateTimeLocal>,
    pub updated_at: Option<DateTimeLocal>,
    pub deleted_at: Option<DateTimeLocal>,
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

impl Model {
    pub fn into_active_base(&self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id.clone()),
            created_at: NotSet,
            updated_at: NotSet,
            deleted_at: NotSet,
            uid: Set(self.uid.clone()),
            name: Set(self.name.clone()),
            description: Set(self.description.clone()),
        }
    }
}
