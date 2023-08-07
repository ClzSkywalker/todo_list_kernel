use std::sync::Arc;

use base::ddd::aggregate::IAggregate;
use common::{contextx::AppContext, i18n::Locale};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Team {
    pub id: String,
    pub uid: String,
    pub name: String,
    pub description: Option<String>,
    // pub classify_ids: Vec<String>,
}

impl IAggregate for Team {}

impl Team {
    pub fn init_data(ctx: Arc<AppContext>, uid: String, tid: String) -> Self {
        let name = match ctx.locale {
            Locale::Zh => String::from("默认团队"),
            Locale::En => String::from("Default team"),
        };
        Self {
            id: tid,
            uid: uid,
            name: name,
            description: None,
        }
    }
}
