use super::super::super::preclude::*;
use base::ddd::repository::IRepository;
pub trait IDevideRepository: IRepository<AG = DevideAggregate, ID = String> {}
