use base::ddd::aggregate::IAggregate;

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
