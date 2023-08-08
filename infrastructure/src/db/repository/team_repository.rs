use base::ddd::repository::IRepository;
use chrono::Local;
use common::contextx::AppContext;
use domain::aggregate::team::repository::item_repository::ITeamRepository;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, QuerySelect, Set};
use sea_query::{Condition, Expr};
use std::sync::Arc;

use super::super::converter::preclude::*;
use super::super::model::preclude::*;
use domain::aggregate::preclude::*;
use sea_orm::prelude::*;

pub struct TeamRepository {
    pub ctx: Arc<AppContext>,
}

#[async_trait::async_trait]
impl IRepository for TeamRepository {
    type AG = TeamAggregate;
    type ID = String;
    async fn insert(&self, ag: Self::AG) -> anyhow::Result<Self::AG> {
        let mut m = TeamSerialize(ag.clone());
        m.created_at = Some(Local::now());
        let active: TeamActiveModel = m.into();
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

        let active: UserTeamActiveModel = UserTeamModel {
            id: common::utils::generate_ulid(),
            created_at: Some(Local::now()),
            updated_at: None,
            deleted_at: None,
            uid: ag.uid.clone(),
            tid: ag.id.clone(),
        }
        .into();

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
        let active = TeamEntity::update(TeamActiveModel {
            id: Set(id.clone()),
            deleted_at: Set(Some(Local::now())),
            ..Default::default()
        })
        .filter(Condition::any().add(Expr::col(TeamColumn::DeletedAt).is_null()));
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
        let m = TeamSerialize(ag.clone());
        let mut active = (&m).into_active_base();
        active.updated_at = Set(Some(Local::now()));

        let active = TeamEntity::update(active)
            .filter(Condition::any().add(Expr::col(TaskColumn::DeletedAt).is_null()));

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
        let active = TeamEntity::find_by_id(id.clone())
            .filter(Condition::any().add(Expr::col(TeamColumn::DeletedAt).is_null()))
            .limit(1);
        let res = match &__self.ctx.tx {
            Some(r) => active.one(r).await,
            None => active.one(&self.ctx.db).await,
        };
        let res = match res {
            Ok(r) => match r {
                Some(r) => r,
                None => return Ok(None),
            },
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, id);
                anyhow::bail!(e)
            }
        };

        Ok(Some(TeamDeserialize(res)))
    }
}

#[async_trait::async_trait]
impl ITeamRepository for TeamRepository {}
