use std::sync::Arc;

use base::ddd::aggregate::IAggregate;
use common::{contextx::AppContext, i18n::Locale};
use serde::Serialize;

use crate::share::value_object::{classify_order_type::OrderType, classify_show_type::ShowType};

#[derive(Debug, Clone, Serialize)]
pub struct Classify {
    pub id: String,
    pub uid: String,
    pub team_id: String,
    pub title: String,
    pub color: String,
    pub show_type: ShowType,
    pub order_type: OrderType,
    pub sort: u32,
    pub parent_id: Option<String>,
    pub devide_ids:Vec<String>,
}

impl IAggregate for Classify {}

impl Classify {
    pub fn init_data(ctx: Arc<AppContext>, uid: String, tid: String) -> Vec<Self> {
        let mut c1 = Self {
            id: common::utils::generate_ulid(),
            uid: uid.clone(),
            team_id: tid.clone(),
            title: "".to_string(),
            color: "#fd8f80".to_string(),
            show_type: ShowType::default(),
            order_type: OrderType::default(),
            sort: 0,
            parent_id: None,
            devide_ids:Vec::new()
        };

        let mut c2 = Self {
            id: common::utils::generate_ulid(),
            uid: uid.clone(),
            team_id: tid.clone(),
            title: "".to_string(),
            color: "#a0cb62".to_string(),
            show_type: ShowType::default(),
            order_type: OrderType::default(),
            sort: 1,
            parent_id: None,
            devide_ids:Vec::new()
        };

        let mut c3 = Self {
            id: common::utils::generate_ulid(),
            uid: uid.clone(),
            team_id: tid.clone(),
            title: "".to_string(),
            color: "#4ac0e4".to_string(),
            show_type: ShowType::default(),
            order_type: OrderType::default(),
            sort: 2,
            parent_id: None,
            devide_ids:Vec::new()
        };

        let mut c4 = Self {
            id: common::utils::generate_ulid(),
            uid: uid.clone(),
            team_id: tid.clone(),
            title: "".to_string(),
            color: "#b4b4b4".to_string(),
            show_type: ShowType::default(),
            order_type: OrderType::default(),
            sort: 3,
            parent_id: None,
            devide_ids:Vec::new()
        };

        let mut c5 = Self {
            id: common::utils::generate_ulid(),
            uid: uid.clone(),
            team_id: tid.clone(),
            title: "".to_string(),
            color: "#b4b4b4".to_string(),
            show_type: ShowType::default(),
            order_type: OrderType::default(),
            sort: 4,
            parent_id: None,
            devide_ids:Vec::new()
        };

        match ctx.locale {
            Locale::Zh => {
                c1.title = String::from("紧急&重要");
                c2.title = String::from("紧急&不重要");
                c3.title = String::from("不紧急&重要");
                c4.title = String::from("不紧急&不重要");
                c5.title = String::from("未分类");
            }
            Locale::En => {
                c1.title = String::from("Important & Urgent");
                c2.title = String::from("Important & Not Urgent");
                c3.title = String::from("Not Important & Urgent");
                c4.title = String::from("Not Important & Not Urgent");
                c5.title = String::from("unclassified");
            }
        };

        vec![c1, c2, c3, c4, c5]
    }
}
