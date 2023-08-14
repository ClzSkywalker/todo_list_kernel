use domain::share::value_object::user_type::{MemberType, RegisterType};
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};

use super::preclude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: Option<DateTimeLocal>,
    pub updated_at: Option<DateTimeLocal>,
    pub deleted_at: Option<DateTimeLocal>,
    pub team_id_port: String,
    pub nick_name: String,
    pub member_type: MemberType,
    pub register_type: RegisterType,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub pwd: String,
    pub version: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    // one to one
    Resource,
    // one to many
    Team,
    // many to many
    UserTeam,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Team => Entity::has_many(TeamEntity).into(),
            Relation::UserTeam => Entity::has_many(UserTeamEntity)
                .from(Column::Id)
                .to(UserTeamColumn::Uid)
                .into(),
            Relation::Resource => Entity::has_one(ResourceEntity).into(),
        }
    }
}

impl Related<UserTeamEntity> for Entity {
    fn to() -> RelationDef {
        Relation::UserTeam.def()
    }
}

impl Related<TeamEntity> for Entity {
    fn to() -> RelationDef {
        UserTeamRelation::Team.def()
    }

    fn via() -> Option<RelationDef> {
        Some(UserTeamRelation::User.def().rev())
    }
}

impl Related<ResourceEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Resource.def()
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
            team_id_port: Set(self.team_id_port.clone()),
            nick_name: Set(self.nick_name.clone()),
            member_type: Set(self.member_type.clone()),
            register_type: Set(self.register_type.clone()),
            picture: Set(self.picture.clone()),
            email: Set(self.email.clone()),
            phone: Set(self.phone.clone()),
            pwd: Set(self.pwd.clone()),
            version: Set(self.version.clone()),
        }
    }
}
