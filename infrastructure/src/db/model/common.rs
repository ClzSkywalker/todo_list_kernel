use common::{errorx::Errorx, i18n::I18nKey};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DbConn};

pub async fn init_db(
    locale: common::i18n::Locale,
    db_path: &String,
) -> Result<DbConn, common::errorx::Errorx> {
    match migration::Migrator::create_db(db_path).await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("{}", e.to_string());
            return Err(Errorx::new(locale, I18nKey::DbCreate));
        }
    }

    let db_path = "sqlite://".to_string() + db_path;
    let db: sea_orm::DatabaseConnection =
        match Database::connect(ConnectOptions::new(db_path)).await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("{}", e.to_string());
                return Err(Errorx::new(
                    locale,
                    I18nKey::DbInit,
                ));
            }
        };

    match Migrator::up(&db, None).await {
        Ok(_) => {}
        Err(_) => {}
    };

    Ok(db)
}
