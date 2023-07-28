use anyhow::Result;
use base::ddd::repository::IRepository;
use sea_orm::prelude::async_trait::async_trait;

use crate::aggregate::task::model::task_content::TaskContent;

#[async_trait]
pub trait ITaskRepository: IRepository {
    // async fn by_user_name(&self, user_name: String) -> Result<Task>;

    async fn insert_content(&self, tc: TaskContent) -> Result<()>;
}
