use chrono::{DateTime, Local};

#[derive(Debug,Clone)]
pub struct Task{
    pub id: i32,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
    pub uuid:String,
    pub created_by:String,
}