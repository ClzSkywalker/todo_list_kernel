use super::aggregate::IAggregate;
use anyhow::Result;

#[async_trait::async_trait]
pub trait IRepository: Sync + Send {
    //
    type AG: IAggregate;
    // 唯一标识
    type ID;
    async fn delete(&self, id: Self::ID) -> Result<()>;
    async fn by_id(&self, id: Self::ID) -> Result<Self::AG>;
    async fn insert(&self, s: Self::AG) -> Result<Self::AG>;
    // async fn save_and_flush(&self, ctx: &AppContext, s: Self::AG) -> Result<Self::AG>;
}
