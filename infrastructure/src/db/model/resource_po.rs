use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};

use super::preclude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "resources")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: Option<DateTimeLocal>,
    pub updated_at: Option<DateTimeLocal>,
    pub deleted_at: Option<DateTimeLocal>,
    pub uid: String,
    pub exp: i64,
    pub gold_coin: i64,
    pub diamond: i64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(UserEntity)
                .from(Column::Id)
                .to(UserColumn::Id)
                .into(),
        }
    }
}

impl Related<UserEntity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
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
            exp: Set(self.exp),
            gold_coin: Set(self.gold_coin),
            diamond: Set(self.diamond),
        }
    }
}
