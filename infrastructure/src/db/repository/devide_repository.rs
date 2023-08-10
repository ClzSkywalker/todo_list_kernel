use base::ddd::repository::IRepository;
use chrono::Local;
use common::contextx::AppContext;
use domain::aggregate::devide::repository::idevide_repository::IDevideRepository;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, QuerySelect, Set};
use sea_query::{Condition, Expr};
use std::sync::Arc;

use super::super::converter::preclude::*;
use super::super::model::preclude::*;
use domain::aggregate::preclude::*;
use sea_orm::prelude::*;

pub struct DevideRepository {
    pub ctx: Arc<AppContext>,
}

#[async_trait::async_trait]
impl IRepository for DevideRepository {
    type AG = DevideAggregate;
    type ID = String;
    async fn insert(&self, ag: Self::AG) -> anyhow::Result<Self::AG> {
        let mut m = DevideSerialize(ag.clone());
        m.created_at = Some(Local::now());
        let active: DevideActiveModel = m.into();
        let res = match &self.ctx.tx {
            Some(r) => active.insert(r).await,
            None => active.insert(&self.ctx.db).await,
        };

        match res {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{},e:{},model:{:?}", self.ctx.to_string(), e, ag);
                anyhow::bail!(e);
            }
        };
        Ok(ag)
    }
    async fn delete(&self, id: Self::ID) -> anyhow::Result<()> {
        let active = DevideEntity::update(DevideActiveModel {
            id: Set(id.clone()),
            deleted_at: Set(Some(Local::now())),
            ..Default::default()
        })
        .filter(Condition::all().add(Expr::col(DevideColumn::DeletedAt).is_null()));
        let res = match &self.ctx.tx {
            Some(r) => active.exec(r).await,
            None => active.exec(&self.ctx.db).await,
        };

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("{},e:{},model:{:?}", self.ctx.to_string(), e, id);
                anyhow::bail!(e);
            }
        }
    }

    async fn update(&self, ag: Self::AG) -> anyhow::Result<()> {
        let m = DevideSerialize(ag.clone());
        let mut active = (&m).into_active_base();
        active.updated_at = Set(Some(Local::now()));

        let active = DevideEntity::update(active)
            .filter(Condition::all().add(Expr::col(TaskColumn::DeletedAt).is_null()));

        let res = match &self.ctx.tx {
            Some(r) => active.exec(r).await,
            None => active.exec(&self.ctx.db).await,
        };

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, ag);
                anyhow::bail!(e)
            }
        }
    }
    async fn by_id(&self, id: Self::ID) -> anyhow::Result<Option<Self::AG>> {
        let active = DevideEntity::find_by_id(id.clone())
            .find_with_related(TaskEntity)
            .filter(Condition::all().add(Expr::col(DevideColumn::DeletedAt).is_null()))
            .limit(1);
        let res = match &__self.ctx.tx {
            Some(r) => active.all(r).await,
            None => active.all(&self.ctx.db).await,
        };
        let res = match res {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, id);
                anyhow::bail!(e)
            }
        };

        let res = match res.last() {
            Some(r) => r.clone(),
            None => {
                return Ok(None);
            }
        };

        Ok(Some(DevideDeserialize(res.0, res.1)))
    }
}

#[async_trait::async_trait]
impl IDevideRepository for DevideRepository {}
