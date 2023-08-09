use chrono::{DateTime, Local};
use domain::share::value_object::{classify_order_type::OrderType, classify_show_type::ShowType};
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "classify")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
    pub uid: String,
    pub team_id: String,
    pub title: String,
    pub color: String,
    pub show_type: ShowType,
    pub order_type: OrderType,
    pub sort: u32,
    pub parent_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Team,
    Classify,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Team => Entity::belongs_to(super::team_po::Entity)
                .from(Column::TeamId)
                .to(super::team_po::Column::Id)
                .into(),
            Relation::Classify => Entity::has_many(super::devide_po::Entity).into(),
        }
    }
}

impl Related<super::devide_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Classify.def()
    }
}

impl Related<super::team_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
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
            team_id: Set(self.team_id.clone()),
            title: Set(self.title.clone()),
            color: Set(self.color.clone()),
            show_type: Set(self.show_type.clone()),
            order_type: Set(self.order_type.clone()),
            sort: Set(self.sort),
            parent_id: Set(self.parent_id.clone()),
        }
    }
}
