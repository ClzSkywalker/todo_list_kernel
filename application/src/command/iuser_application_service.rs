use base::ddd::application_service::IApplicationService;

use crate::query::model::common::dto::RespToken;

#[async_trait::async_trait]
pub trait IUserApplicationService: IApplicationService {
    async fn create_by_id(&mut self) -> anyhow::Result<RespToken>;
}
