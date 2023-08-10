use std::sync::Arc;

use base::ddd::repository::IRepository;
use chrono::Local;
use common::contextx::AppContext;
use domain::aggregate::{preclude::*, user::repository::iuser_repository::IUserRepository};
use migration::Expr;
use sea_orm::{ActiveModelTrait, Condition, EntityTrait, QueryFilter, QuerySelect, Set};

use super::super::model::preclude::*;
use crate::db::converter::user_converter;

pub struct UserRepository {
    pub ctx: Arc<AppContext>,
}

#[async_trait::async_trait]
impl IRepository for UserRepository {
    type AG = UserAggregate;
    type ID = String;
    async fn insert(&self, ag: Self::AG) -> anyhow::Result<Self::AG> {
        let mut m = user_converter::serialize(ag.clone());
        m.created_at = Some(Local::now());
        let active: UserActiveModel = m.into();
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
        let active = UserEntity::update(UserActiveModel {
            id: Set(id.clone()),
            deleted_at: Set(Some(Local::now())),
            ..Default::default()
        })
        .filter(Condition::all().add(Expr::col(UserColumn::DeletedAt).is_null()));
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
        let m = user_converter::serialize(ag.clone());
        let mut active = (&m).into_active_base();
        active.updated_at = Set(Some(Local::now()));

        let active = UserEntity::update(active)
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
        let active = UserEntity::find_by_id(id.clone())
            .find_with_related(TeamEntity)
            .filter(
                Condition::all()
                    .add(Expr::col((UserEntity, TaskColumn::DeletedAt)).is_null())
                    .add(Expr::col((TeamEntity, TeamColumn::DeletedAt)).is_null()),
            )
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
            None => return Ok(None),
        };

        Ok(Some(user_converter::deserialize(res.0, res.1)))
    }
}

#[async_trait::async_trait]
impl IUserRepository for UserRepository {
    async fn find_by_email(&self, email: String) -> anyhow::Result<Option<UserAggregate>> {
        let active = UserEntity::find()
            .find_with_related(TeamEntity)
            .filter(
                Condition::all()
                    .add(Expr::col((UserEntity, UserColumn::Email)).eq(email.clone()))
                    .add(Expr::col((UserEntity, TaskColumn::DeletedAt)).is_null())
                    .add(Expr::col((TeamEntity, TeamColumn::DeletedAt)).is_null()),
            )
            .limit(1);
        let res = match &self.ctx.tx {
            Some(r) => active.all(r).await,
            None => active.all(&self.ctx.db).await,
        };
        let res = match res {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, email);
                anyhow::bail!(e)
            }
        };

        let res = match res.last() {
            Some(r) => r.clone(),
            None => return Ok(None),
        };

        Ok(Some(user_converter::deserialize(res.0, res.1)))
    }
}
