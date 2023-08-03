use base::ddd::repository::IRepository;
use sea_orm::prelude::async_trait::async_trait;

use crate::aggregate::task::model::{task::Task, task_content::TaskContent};

#[async_trait]
pub trait ITaskRepository: IRepository<AG = Task, ID = String> {
    async fn content_insert(&self, tc: TaskContent) -> anyhow::Result<()>;
    async fn content_delete(&self, id: String) -> anyhow::Result<()>;
    async fn content_update(&self, tc: TaskContent) -> anyhow::Result<()>;
    async fn content_first_by_id(&self, id: String) -> anyhow::Result<Option<TaskContent>>;
}
