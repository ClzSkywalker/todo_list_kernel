use base::ddd::repository::IRepository;

use super::super::super::preclude::*;

#[async_trait::async_trait]
pub trait IClassifyRepository: IRepository<AG = ClassifyAggregate, ID = String> {
    async fn insert_many(&self, ags: Vec<ClassifyAggregate>) -> anyhow::Result<()>;
}
