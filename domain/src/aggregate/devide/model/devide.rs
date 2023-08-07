use std::sync::Arc;

use base::ddd::aggregate::IAggregate;
use common::{contextx::AppContext, i18n::Locale};

#[derive(Debug, Clone)]
pub struct Devide {
    pub id: String,
    pub classify_id: String,
    pub uid: String,
    pub title: String,
    pub sort: u32,
    pub task_ids: Vec<String>,
}

impl IAggregate for Devide {}

impl Devide {
    pub fn init_data(ctx: Arc<AppContext>, uid: String, cid: String) -> Self {
        let title = match ctx.locale {
            Locale::Zh => String::from("未分类"),
            Locale::En => String::from("Not classified"),
        };
        Self {
            id: common::utils::generate_ulid(),
            classify_id: cid,
            uid: uid,
            title: title,
            sort: 0,
            task_ids: Vec::new(),
        }
    }
}
