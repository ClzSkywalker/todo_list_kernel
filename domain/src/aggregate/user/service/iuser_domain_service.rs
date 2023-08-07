use base::ddd::domain_service::IDomainService;

use super::super::super::preclude::*;

#[async_trait::async_trait]
pub trait IUserDomainService: IDomainService {
    async fn register(&self) -> anyhow::Result<UserAggregate>;
}
