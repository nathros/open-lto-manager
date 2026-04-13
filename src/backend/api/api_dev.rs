use dioxus::prelude::*;

#[get("/api/dev/db/backup/list")]
pub async fn dev_db_list() -> Result<Vec<String>> {
    use crate::backend::dev::database_backup_restore::dev_database_list;

    Ok(dev_database_list())
}

#[get("/api/dev/db/backup/{dir}")]
pub async fn dev_db_backup(dir: String) -> Result<bool> {
    use crate::backend::dev::database_backup_restore::dev_database_backup;

    Ok(dev_database_backup(dir))
}

#[get("/api/dev/db/restore/{dir}")]
pub async fn dev_db_restore(dir: String) -> Result<bool> {
    use crate::backend::dev::database_backup_restore::dev_database_restore;

    Ok(dev_database_restore(dir).unwrap_or(false))
}
