use base::ddd::value_object::IValueObject;
use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct TaskDate {
    pub completed_at: Option<DateTime<Local>>,
    pub give_up_at: Option<DateTime<Local>>,
    pub start_at: Option<DateTime<Local>>,
    pub end_at: Option<DateTime<Local>>,
}
impl IValueObject for TaskDate {}
