use std::sync::Arc;

use base::ddd::aggregate::IAggregate;
use common::{config::VERSION, contextx::AppContext, errorx::Errorx, i18n::Locale, jwt, utils};
use serde::Serialize;

use crate::share::value_object::user_type::{MemberType, RegisterType};

use super::team::Team;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: String,
    pub nick_name: String,
    pub member_type: MemberType,
    pub register_type: RegisterType,
    pub team_id_port: String,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub pwd: String,
    pub team_list: Vec<Team>,
    pub version: String,
}
impl IAggregate for User {}

impl User {
    pub fn init_data(ctx: Arc<AppContext>) -> Self {
        let tid = common::utils::generate_ulid();
        match ctx.locale {
            Locale::Zh => Self {
                id: common::utils::generate_ulid(),
                nick_name: String::from("花猫"),
                member_type: MemberType::Normal,
                register_type: RegisterType::Uid,
                team_id_port: tid.clone(),
                picture: None,
                email: None,
                phone: None,
                pwd: String::from(""),
                team_list: vec![Team {
                    id: tid,
                    name: String::from(""),
                }],
                version: VERSION.to_string(),
            },
            Locale::En => Self {
                id: common::utils::generate_ulid(),
                nick_name: String::from("hi"),
                member_type: MemberType::Normal,
                register_type: RegisterType::Uid,
                team_id_port: tid.clone(),
                picture: None,
                email: None,
                phone: None,
                pwd: String::from(""),
                team_list: vec![Team {
                    id: tid,
                    name: String::from(""),
                }],
                version: VERSION.to_string(),
            },
        }
    }

    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-08-10
    /// Description    : 创建jwt token
    /// param           {*} self
    /// param           {Locale} locale
    /// return          {*}
    ///    
    pub fn generate_token(&self, locale: Locale) -> anyhow::Result<String> {
        match jwt::generate_token(self.id.clone(), self.team_id_port.clone()) {
            Ok(r) => Ok(r),
            Err(_) => {
                anyhow::bail!(Errorx::new(locale, common::i18n::I18nKey::TokenGenerate))
            }
        }
    }

    pub fn pwd_bcrypt(&mut self, locale: Locale) -> anyhow::Result<()> {
        match utils::bcrypt_hash(self.pwd.clone()) {
            Ok(r) => {
                self.pwd = r;
            }
            Err(_) => {
                anyhow::bail!(Errorx::new(locale, common::i18n::I18nKey::EncryptionError))
            }
        };
        Ok(())
    }

    pub fn pwd_varify(&self, locale: Locale, pwd: String) -> anyhow::Result<bool> {
        match utils::bcrypt_verify(pwd, self.pwd.clone()) {
            Ok(r) => {
                if !r {
                    return Ok(false);
                }
                Ok(true)
            }
            Err(_) => {
                anyhow::bail!(Errorx::new(locale, common::i18n::I18nKey::UserLogin))
            }
        }
    }
}
