use std::sync::Arc;

use super::super::super::super::preclude::*;
use base::ddd::domain_service::IDomainService;
use common::{contextx::AppContext, errorx::Errorx, i18n::I18nKey};

use crate::aggregate::{
    classify::repository::iclassify_repository::IClassifyRepository,
    devide::repository::idevide_repository::IDevideRepository,
    task::repository::itask_repository::ITaskRepository,
    task_mode::repository::itask_mode_repository::ITaskModeRepository,
    team::repository::item_repository::ITeamRepository,
    user::{
        repository::iuser_repository::IUserRepository,
        service::iuser_domain_service::IUserDomainService,
    },
};

pub struct UserDomainService<UR, TR, CR, TMR, DR, TASKR>
where
    UR: IUserRepository,
    TR: ITeamRepository,
    CR: IClassifyRepository,
    TMR: ITaskModeRepository,
    DR: IDevideRepository,
    TASKR: ITaskRepository,
{
    pub ctx: Arc<AppContext>,
    pub user_repostitory: UR,
    pub team_repository: TR,
    pub classify_repository: CR,
    pub task_mode_repository: TMR,
    pub devide_repository: DR,
    pub task_repository: TASKR,
}

impl<UR, TR, CR, TMR, DR, TASKR> IDomainService for UserDomainService<UR, TR, CR, TMR, DR, TASKR>
where
    UR: IUserRepository,
    TR: ITeamRepository,
    CR: IClassifyRepository,
    TMR: ITaskModeRepository,
    DR: IDevideRepository,
    TASKR: ITaskRepository,
{
}

#[async_trait::async_trait]
impl<UR, TR, CR, TMR, DR, TASKR> IUserDomainService
    for UserDomainService<UR, TR, CR, TMR, DR, TASKR>
where
    UR: IUserRepository,
    TR: ITeamRepository,
    CR: IClassifyRepository,
    TMR: ITaskModeRepository,
    DR: IDevideRepository,
    TASKR: ITaskRepository,
{
    async fn register(&self) -> anyhow::Result<UserAggregate> {
        let u = UserAggregate::init_data(self.ctx.clone());
        let u = match __self.user_repostitory.insert(u).await {
            Ok(r) => r,
            Err(_) => {
                anyhow::bail!(Errorx::new(self.ctx.locale, I18nKey::UserCreate))
            }
        };

        let t = TeamAggregate::init_data(self.ctx.clone(), u.id.clone(), u.team_id_port.clone());
        match __self.team_repository.insert(t).await {
            Ok(r) => r,
            Err(_) => {
                anyhow::bail!(Errorx::new(self.ctx.locale, I18nKey::UserCreate))
            }
        };

        let c =
            ClassifyAggregate::init_data(self.ctx.clone(), u.id.clone(), u.team_id_port.clone());
        match __self.classify_repository.insert_many(c.clone()).await {
            Ok(_) => {}
            Err(_) => {
                anyhow::bail!(Errorx::new(self.ctx.locale, I18nKey::UserCreate))
            }
        };

        let tm = TaskModeAggregate::init_data(u.id.clone());
        match self.task_mode_repository.insert(tm.clone()).await {
            Ok(_) => {}
            Err(_) => {
                anyhow::bail!(Errorx::new(self.ctx.locale, I18nKey::UserCreate))
            }
        };

        let devide = DevideAggregate::init_data(
            self.ctx.clone(),
            u.id.clone(),
            c.first().unwrap().id.clone(),
        );

        match self.devide_repository.insert(devide.clone()).await {
            Ok(_) => {}
            Err(_) => {
                anyhow::bail!(Errorx::new(self.ctx.locale, I18nKey::UserCreate))
            }
        };

        let task = TaskAggregate::init_data(
            self.ctx.clone(),
            u.id.clone(),
            devide.id.clone(),
            tm.id.clone(),
        );
        match self.task_repository.insert(task).await {
            Ok(_) => {}
            Err(_) => {
                anyhow::bail!(Errorx::new(self.ctx.locale, I18nKey::UserCreate))
            }
        };

        Ok(u)
    }
}
