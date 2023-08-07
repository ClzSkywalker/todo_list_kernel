use chrono::{DateTime, Local};
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "devide")]
#[sea_query::enum_def]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
    pub classify_id: String,
    pub uid: String,
    pub title: String,
    pub sort: u32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Classify,
    Task,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Task => Entity::has_many(super::task_po::Entity).into(),
            Relation::Classify => Entity::belongs_to(super::classify_po::Entity)
                .from(Column::ClassifyId)
                .to(super::classify_po::Column::Id)
                .into(),
        }
    }
}

impl Related<super::task_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
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
            classify_id: Set(self.classify_id.clone()),
            title: Set(self.title.clone()),
            sort: Set(self.sort),
        }
    }
}
