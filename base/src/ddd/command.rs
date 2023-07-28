use anyhow::Result;

#[async_trait::async_trait]
pub trait ICommand {
    /// result
    type R;
    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-07-28
    /// Description    : 能力点执行前的参数校验
    /// param           {*} self
    /// return          {*}
    ///    
    async fn check_handler(&self) -> Result<()>;

    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-07-28
    /// Description    : 能力点执行前的幂等校验,当前能力点已执行，不再执行业务逻辑、true：当前能力点未执行，继续执行业务逻辑
    /// param           {*} self
    /// return          {*}
    ///    
    async fn check_idempotent(&self) -> Result<bool>;

    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-07-28
    /// Description    : 执行
    /// param           {*} self
    /// return          {*}
    ///    
    async fn execute(&self) -> Result<Self::R>;
}
