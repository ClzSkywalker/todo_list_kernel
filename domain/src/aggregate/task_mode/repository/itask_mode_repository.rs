use super::super::super::preclude::*;
use base::ddd::repository::IRepository;

#[async_trait::async_trait]
pub trait ITaskModeRepository: IRepository<AG = TaskModeAggregate, ID = String> {}
