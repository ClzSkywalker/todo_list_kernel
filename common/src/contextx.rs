use sea_orm::{DatabaseConnection, DatabaseTransaction};
use std::{
    fmt::Display,
    sync::{Arc, RwLock},
};

use crate::i18n::Locale;

pub type SharedStateCtx = Arc<RwLock<AppContext>>;

#[derive(Debug)]
pub struct AppContext {
    pub db: DatabaseConnection,
    pub tx: Option<DatabaseTransaction>,
    pub uid: String,
    pub tid: String,
    pub locale: Locale,
}

impl AppContext {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            tx: None,
            uid: "".to_string(),
            tid: "".to_string(),
            locale: Locale::En,
        }
    }
}

impl Display for AppContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ctx:{{uid:{},tid:{},locale:{})}}",
            self.uid, self.tid, self.locale
        )
    }
}

impl Clone for AppContext {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            tx: None,
            uid: self.uid.clone(),
            tid: self.tid.clone(),
            locale: self.locale.clone(),
        }
    }
}