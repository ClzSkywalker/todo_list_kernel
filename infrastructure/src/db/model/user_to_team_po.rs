use sea_orm::entity::prelude::*;
use sea_query::enum_def;

use super::preclude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "user_to_team")]
#[enum_def]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: Option<DateTimeLocal>,
    pub updated_at: Option<DateTimeLocal>,
    pub deleted_at: Option<DateTimeLocal>,
    pub uid: String,
    pub tid: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    Team,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(UserEntity)
                .from(Column::Uid)
                .to(UserColumn::Id)
                .into(),
            Relation::Team => Entity::belongs_to(TeamEntity)
                .from(Column::Tid)
                .to(TeamColumn::Id)
                .into(),
        }
    }
}

impl Related<TeamEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl Related<UserEntity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
