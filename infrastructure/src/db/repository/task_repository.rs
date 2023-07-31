use anyhow::{anyhow, Result};
use base::ddd::repository::IRepository;
use chrono::Local;
use common::contextx::AppContext;
use domain::aggregate::task::{
    model::{task::Task, task_content::TaskContent},
    repository::itask_repository::ITaskRepository,
};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet};
use std::sync::Arc;

use super::super::model::preclude::*;
use crate::db::converter::{task_content_converter, task_converter};

pub struct TaskRepository {
    pub ctx: Arc<AppContext>,
}

impl TaskRepository {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        TaskRepository { ctx: ctx }
    }
}

#[async_trait::async_trait]
impl IRepository for TaskRepository {
    type AG = Task;

    type ID = String;

    async fn delete(&self, id: Self::ID) -> Result<()> {
        todo!()
    }

    async fn by_id(&self, id: Self::ID) -> Result<Self::AG> {
        todo!()
    }

    async fn insert(&self, s: Self::AG) -> Result<Self::AG> {
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
                return Err(anyhow!(e));
            }
        };
        Ok(s)
    }
}

#[async_trait::async_trait]
impl ITaskRepository for TaskRepository {
    // async fn by_user_name(&self, user_name: String) -> Result<Task> {
    //     Err(anyhow!(""))
    // }

    async fn insert_content(&self, tc: TaskContent) -> Result<()> {
        let mut m = task_content_converter::serialize(tc.clone());
        m.created_at = Some(Local::now());
        let mut am: TaskContentActiveModel = m.clone().into();
        am.id = NotSet;
        let res = match &self.ctx.tx {
            Some(r) => am.insert(r),
            None => am.insert(&self.ctx.db),
        };
        match res.await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, tc);
                return Err(anyhow!(e));
            }
        }
        Ok(())
    }
}
