use base::ddd::repository::IRepository;

use crate::aggregate::user::model::user::User;

#[async_trait::async_trait]
pub trait IUserRepository: IRepository<AG = User, ID = String> {
    // async fn content_insert(&self, tc: TaskContent) -> anyhow::Result<()>;
    // async fn content_delete(&self, id: String) -> anyhow::Result<()>;
    // async fn content_update(&self, tc: TaskContent) -> anyhow::Result<()>;
    // async fn content_first_by_id(&self, id: String) -> anyhow::Result<Option<TaskContent>>;
}
