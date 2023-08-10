use base::ddd::repository::IRepository;
use chrono::Local;
use common::contextx::AppContext;
use domain::aggregate::task::repository::itask_repository::ITaskRepository;
use migration::Expr;
use sea_orm::{ActiveModelTrait, Condition, EntityTrait, QueryFilter, Set};
use std::sync::Arc;

use super::super::converter::preclude::*;
use super::super::model::preclude::*;
use domain::aggregate::preclude::*;

pub struct TaskRepository {
    pub ctx: Arc<AppContext>,
}

#[async_trait::async_trait]
impl IRepository for TaskRepository {
    type AG = TaskAggregate;

    type ID = String;

    async fn insert(&self, s: Self::AG) -> anyhow::Result<Self::AG> {
        let mut m = TaskSerialize(s.clone());
        m.created_at = Some(Local::now());
        let am: TaskActiveModel = m.into();
        let res = match &self.ctx.tx {
            Some(r) => am.insert(r).await,
            None => am.insert(&self.ctx.db).await,
        };

        match res {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{},e:{},model:{:?}", self.ctx.to_string(), e, s);
                anyhow::bail!(e);
            }
        };

        let mut m = TaskContentSerialize(s.task_content.clone());
        m.created_at = Some(Local::now());
        let am: TaskContentActiveModel = m.clone().into();
        let res = match &self.ctx.tx {
            Some(r) => am.insert(r).await,
            None => am.insert(&self.ctx.db).await,
        };

        match res {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, s.task_content.clone());
                anyhow::bail!(e);
            }
        };

        Ok(s)
    }

    async fn delete(&self, id: Self::ID) -> anyhow::Result<()> {
        let active = TaskEntity::update(TaskActiveModel {
            id: Set(id.clone()),
            deleted_at: Set(Some(Local::now())),
            ..Default::default()
        })
        .filter(Condition::any().add(Expr::col(TaskColumn::DeletedAt).is_null()));
        let res = match &self.ctx.tx {
            Some(r) => active.exec(r).await,
            None => active.exec(&self.ctx.db).await,
        };

        let task = match res {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("{},e:{},model:{:?}", self.ctx.to_string(), e, id);
                anyhow::bail!(e);
            }
        };

        let m = TaskContentActiveModel {
            id: Set(task.content_id.clone()),
            updated_at: Set(Some(Local::now())),
            ..Default::default()
        };
        let active = TaskContentEntity::update(m)
            .filter(Condition::any().add(Expr::col(TaskColumn::DeletedAt).is_null()));

        let r = match &self.ctx.tx {
            Some(r) => active.exec(r).await,
            None => active.exec(&self.ctx.db).await,
        };

        match r {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, task.content_id.clone());
                anyhow::bail!(e)
            }
        }
    }

    async fn update(&self, ag: Self::AG) -> anyhow::Result<()> {
        let m = TaskSerialize(ag.clone());
        let mut active = (&m).into_active_base();
        active.updated_at = Set(Some(Local::now()));

        let active = TaskEntity::update(active)
            .filter(Condition::any().add(Expr::col(TaskColumn::DeletedAt).is_null()));

        let res = match &self.ctx.tx {
            Some(r) => active.exec(r).await,
            None => active.exec(&self.ctx.db).await,
        };

        match res {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, ag);
                anyhow::bail!(e)
            }
        };

        let mut m = TaskContentSerialize(ag.task_content.clone());
        m.updated_at = Some(Local::now());
        let mut active: TaskContentActiveModel = m.into();
        active.not_set(TaskContentColumn::CreatedAt);
        let res = match &self.ctx.tx {
            Some(r) => active.update(r).await,
            None => active.update(&self.ctx.db).await,
        };
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, ag.task_content.clone());
                anyhow::bail!(e);
            }
        }
    }

    async fn by_id(&self, id: Self::ID) -> anyhow::Result<Option<Self::AG>> {
        let active = TaskEntity::find_by_id(id.clone())
            .filter(Condition::any().add(Expr::col(TaskColumn::DeletedAt).is_null()));
        let res = match &__self.ctx.tx {
            Some(r) => active.one(r).await,
            None => active.one(&self.ctx.db).await,
        };
        let res = match res {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, id);
                anyhow::bail!(e)
            }
        };

        let task = match res {
            Some(r) => r,
            None => return Ok(None),
        };

        let active = TaskContentEntity::find_by_id(task.content_id.clone());
        let res = match &self.ctx.tx {
            Some(r) => active.one(r).await,
            None => active.one(&self.ctx.db).await,
        };
        // let res = active.one(&self.ctx.db);
        let content = match res {
            Ok(r) => match r {
                Some(r) => {
                    let r = TaskContentDeserialize(r);
                    r
                }
                None => return Ok(None),
            },
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, id);
                anyhow::bail!(e);
            }
        };

        let content = TaskContentSerialize(content);
        Ok(Some(TaskDeserialize(task, content)))
    }
}

#[async_trait::async_trait]
impl ITaskRepository for TaskRepository {}
