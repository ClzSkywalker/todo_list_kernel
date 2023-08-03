use std::sync::Arc;

use base::ddd::repository::IRepository;
use common::contextx::AppContext;
use domain::aggregate::user::{model::user::User, repository::iuser_repository::IUserRepository};

pub struct UserRepository {
    pub ctx: Arc<AppContext>,
}

#[async_trait::async_trait]
impl IRepository for UserRepository {
    type AG = User;
    type ID = String;
    async fn insert(&self, s: Self::AG) -> anyhow::Result<Self::AG> {
        todo!()
    }
    async fn delete(&self, id: Self::ID) -> anyhow::Result<()> {
        todo!()
    }
    async fn update(&self, ag: Self::AG) -> anyhow::Result<()> {
        todo!()
    }
    async fn by_id(&self, id: Self::ID) -> anyhow::Result<Option<Self::AG>> {
        todo!()
    }
}

impl IUserRepository for UserRepository {}
