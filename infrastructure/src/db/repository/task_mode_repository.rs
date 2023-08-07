use std::sync::Arc;

use base::ddd::repository::IRepository;
use chrono::Local;
use common::contextx::AppContext;
use domain::aggregate::task_mode::repository::itask_mode_repository::ITaskModeRepository;
use migration::Expr;
use sea_orm::{ActiveModelTrait, Condition, EntityTrait, QueryFilter, QuerySelect, Set};

use super::super::converter::preclude::*;
use super::super::model::preclude::*;
use domain::aggregate::preclude::*;
use sea_orm::prelude::*;

pub fn new_tasl_mode_repostiory(
    ctx: Arc<AppContext>,
) -> impl ITaskModeRepository<AG = TaskModeAggregate, ID = String> {
    TaskModeRepository { ctx: ctx }
}

pub struct TaskModeRepository {
    pub ctx: Arc<AppContext>,
}

#[async_trait::async_trait]
impl IRepository for TaskModeRepository {
    //
    type AG = TaskModeAggregate;
    // 唯一标识
    type ID = String;
    async fn insert(&self, ag: Self::AG) -> anyhow::Result<Self::AG> {
        let mut m = TaskModeSerialize(ag.clone());
        m.created_at = Some(Local::now());
        let active: TaskModeActiveModel = m.into();
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
        let active = TaskModeEntity::update(TaskModeActiveModel {
            id: Set(id.clone()),
            deleted_at: Set(Some(Local::now())),
            ..Default::default()
        })
        .filter(Condition::any().add(Expr::col(TaskModeColumn::DeletedAt).is_null()));
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
        let m = TaskModeSerialize(ag.clone());
        let mut active = (&m).into_active_base();
        active.updated_at = Set(Some(Local::now()));

        let active = TaskModeEntity::update(active)
            .filter(Condition::any().add(Expr::col(TaskModeColumn::DeletedAt).is_null()));

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
        let active = TaskModeEntity::find_by_id(id.clone())
            .filter(Condition::any().add(Expr::col(TaskModeColumn::DeletedAt).is_null()))
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

        Ok(Some(TaskModeDeserialize(res)))
    }
}

#[async_trait::async_trait]
impl ITaskModeRepository for TaskModeRepository {}
