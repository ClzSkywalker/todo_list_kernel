use base::ddd::repository::IRepository;
use sea_orm::prelude::async_trait::async_trait;

use crate::aggregate::task::model::task::Task;

#[async_trait]
pub trait ITaskRepository: IRepository<AG = Task, ID = String> {}
