use base::ddd::repository::IRepository;
use chrono::Local;
use common::contextx::AppContext;
use domain::aggregate::task::{
    model::{task::Task, task_content::TaskContent},
    repository::itask_repository::ITaskRepository,
};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, EntityTrait};
use std::sync::Arc;

use super::super::model::preclude::*;
use crate::db::converter::{task_content_converter, task_converter};

pub fn new_task_repostiory(ctx: Arc<AppContext>) -> impl ITaskRepository<AG = Task, ID = String> {
    TaskRepository { ctx: ctx }
}

pub struct TaskRepository {
    pub ctx: Arc<AppContext>,
}

#[async_trait::async_trait]
impl IRepository for TaskRepository {
    type AG = Task;

    type ID = String;

    async fn insert(&self, s: Self::AG) -> anyhow::Result<Self::AG> {
        let mut m = task_converter::serialize(s.clone());
        m.created_at = Some(Local::now());
        let mut am: TaskActiveModel = m.clone().into();
        am.id = NotSet;
        let res = match &self.ctx.tx {
            Some(r) => am.insert(r),
            None => am.insert(&self.ctx.db),
        };

        match res.await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{},e:{},model:{:?}", self.ctx.to_string(), e, s);
                anyhow::bail!(e);
            }
        };
        Ok(s)
    }

    async fn delete(&self, id: Self::ID) -> anyhow::Result<()> {
        let active = TaskEntity::delete_by_id(id.clone());
        let res = match &__self.ctx.tx {
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
        let active: TaskActiveModel = task_converter::serialize(ag.clone()).into();
        let res = match &self.ctx.tx {
            Some(r) => active.update(r).await,
            None => active.update(&self.ctx.db).await,
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
        let active = TaskEntity::find_by_id(id.clone());
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

        let content = match __self.content_first_by_id(task.content_id.clone()).await {
            Ok(r) => match r {
                Some(r) => r,
                None => return Ok(None),
            },
            Err(e) => {
                anyhow::bail!(e)
            }
        };

        let content = task_content_converter::serialize(content);
        Ok(Some(task_converter::deserialize(task, content)))
    }
}

#[async_trait::async_trait]
impl ITaskRepository for TaskRepository {
    async fn first_by_id(&self, id: String) -> anyhow::Result<Option<Task>> {
        let active = TaskEntity::find_by_id(id.clone());
        let res = match &self.ctx.tx {
            Some(r) => active.one(r).await,
            None => active.one(&self.ctx.db).await,
        };
        let r = match res {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, id);
                anyhow::bail!(e);
            }
        };

        let task = match r {
            Some(r) => r,
            None => return Ok(None),
        };

        let content = match self.content_first_by_id(task.id.clone()).await {
            Ok(r) => match r {
                Some(r) => r,
                None => return Ok(None),
            },
            Err(e) => {
                anyhow::bail!(e)
            }
        };

        let content = task_content_converter::serialize(content);

        Ok(Some(task_converter::deserialize(task, content)))
    }

    async fn content_insert(&self, tc: TaskContent) -> anyhow::Result<()> {
        let mut m = task_content_converter::serialize(tc.clone());
        m.created_at = Some(Local::now());
        let mut am: TaskContentActiveModel = m.clone().into();
        am.id = NotSet;
        let res = match &self.ctx.tx {
            Some(r) => am.insert(r),
            None => am.insert(&self.ctx.db),
        };
        match res.await {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, tc);
                anyhow::bail!(e);
            }
        }
    }

    async fn content_delete(&self, id: String) -> anyhow::Result<()> {
        let content = match self.content_first_by_id(id.clone()).await {
            Ok(r) => match r {
                Some(r) => r,
                None => return Ok(()),
            },
            Err(e) => {
                anyhow::bail!(e)
            }
        };

        let operator = TaskContentEntity::delete_by_id(content.id);
        let r = match &self.ctx.tx {
            Some(r) => operator.exec(r).await,
            None => operator.exec(&self.ctx.db).await,
        };

        match r {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, id);
                anyhow::bail!(e)
            }
        }
    }

    async fn content_update(&self, tc: TaskContent) -> anyhow::Result<()> {
        let m = task_content_converter::serialize(tc.clone());
        let active: TaskContentActiveModel = m.into();
        let res = match &self.ctx.tx {
            Some(r) => active.update(r).await,
            None => active.update(&self.ctx.db).await,
        };
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, tc);
                anyhow::bail!(e);
            }
        }
    }

    async fn content_first_by_id(&self, id: String) -> anyhow::Result<Option<TaskContent>> {
        let active = TaskContentEntity::find_by_id(id.clone());
        let res = match &self.ctx.tx {
            Some(r) => active.one(r).await,
            None => active.one(&self.ctx.db).await,
        };
        // let res = active.one(&self.ctx.db);
        match res {
            Ok(r) => match r {
                Some(r) => {
                    let r = task_content_converter::deserialize(r);
                    Ok(Some(r))
                }
                None => Ok(None),
            },
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, id);
                anyhow::bail!(e);
            }
        }
    }
}
