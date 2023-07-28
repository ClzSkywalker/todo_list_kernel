use base::ddd::repository::IRepository;
use domain::aggregate::task::repository::itask_content_repository::ITaskContentRepository;

// pub struct TaskContentRepository {}

// #[async_trait::async_trait]
// impl IRepository for TaskContentRepository {
//     type AG: IAggregate;
//     // 唯一标识
//     type ID;
//     async fn delete(&self, ctx: &AppContext, id: Self::ID) -> Result<()>;
//     async fn by_id(&self, ctx: &AppContext, id: Self::ID) -> Result<Self::AG>;
//     async fn save(&self, ctx: &AppContext, s: Self::AG) -> Result<Self::AG>;
// }

// #[async_trait::async_trait]
// impl ITaskContentRepository for TaskContentRepository {}
