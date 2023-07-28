use base::ddd::repository::IRepository;
use sea_orm::prelude::async_trait;

#[async_trait::async_trait]
pub trait ITaskContentRepository: IRepository {

}