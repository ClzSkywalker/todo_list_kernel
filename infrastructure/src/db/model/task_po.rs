use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};
use sea_query::enum_def;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[enum_def]
#[sea_orm(table_name = "task")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(created_at)]
    pub created_at: Option<DateTimeLocal>,
    #[sea_orm(updated_at)]
    pub updated_at: Option<DateTimeLocal>,
    #[sea_orm(deleted_at)]
    pub deleted_at: Option<DateTimeLocal>,
    pub uid: String,
    pub devide_id: String,
    pub content_id: String,
    pub task_mode_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeLocal>,
    pub give_up_at: Option<DateTimeLocal>,
    pub start_at: Option<DateTimeLocal>,
    pub end_at: Option<DateTimeLocal>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Devide,
    TaskContent,
    TaskMode,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::TaskContent => Entity::has_many(super::task_content_po::Entity).into(),
            Relation::TaskMode => Entity::has_many(super::task_mode_po::Entity).into(),
            Relation::Devide => Entity::belongs_to(super::devide_po::Entity)
                .from(Column::DevideId)
                .to(super::devide_po::Column::Id)
                .into(),
        }
    }
}

impl Related<super::devide_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Devide.def()
    }
}

impl Related<super::task_content_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskContent.def()
    }
}

impl Related<super::task_mode_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskMode.def()
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
            devide_id: Set(self.devide_id.clone()),
            content_id: Set(self.content_id.clone()),
            task_mode_id: Set(self.task_mode_id.clone()),
            parent_id: Set(self.parent_id.clone()),
            title: Set(self.title.clone()),
            completed_at: Set(self.completed_at.clone()),
            give_up_at: Set(self.give_up_at.clone()),
            start_at: Set(self.start_at.clone()),
            end_at: Set(self.end_at.clone()),
        }
    }
}
