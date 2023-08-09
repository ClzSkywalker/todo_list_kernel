use crate::aggregate::preclude::*;
use crate::aggregate::user::model::user::User;
use base::ddd::repository::IRepository;

#[async_trait::async_trait]
pub trait IUserRepository: IRepository<AG = User, ID = String> {
    async fn find_by_email(&self, email: String) -> anyhow::Result<Option<UserAggregate>>;
}
