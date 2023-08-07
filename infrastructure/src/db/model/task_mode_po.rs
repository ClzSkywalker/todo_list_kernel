use common::DateTimeLocal;
use domain::share::value_object::{task_mode_config::ModeConfig, task_mode_type::ModeType};
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};
use sea_query::enum_def;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "task_mode")]
#[enum_def]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: Option<DateTimeLocal>,
    pub updated_at: Option<DateTimeLocal>,
    pub deleted_at: Option<DateTimeLocal>,
    pub uid: String,
    pub config: Option<ModeConfig>,
    pub mode_type: ModeType,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Task,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Task => Entity::belongs_to(super::task_po::Entity)
                .from(Column::Id)
                .to(super::task_po::Column::Id)
                .into(),
        }
    }
}

impl Related<super::task_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
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
            config: Set(self.config.clone()),
            mode_type: Set(self.mode_type.clone()),
        }
    }
}
