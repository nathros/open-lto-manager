use dioxus::prelude::*;

use crate::shared::models::database::label_preset::model_label_preset::{
    LabelOptions, RecordLabelPreset,
};

#[get("/api/label_preset", auth: crate::backend::auth::SessionId)]
pub async fn get_user_preset() -> Result<Vec<RecordLabelPreset>> {
    use std::io::Error;

    use crate::backend::{
        auth::Session, database::db::DB, database::tables::setting::table_setting::TableSetting,
    };

    if let Some(session) = Session::find(&auth) {
        return DB.with(|db| {
            match TableSetting::<RecordLabelPreset>::get_all(db, session.user_id) {
                Ok(records) => Ok(records),
                Err(e) => Err(e)?,
            }
        });
    }

    Err(Error::other("Cannot get user").into())
}

#[post("/api/label_preset", auth: crate::backend::auth::SessionId)]
pub async fn save_label_preset(name: String, options: LabelOptions) -> Result<bool> {
    use std::io::Error;

    use crate::backend::{
        auth::Session,
        database::db::DB,
        database::tables::{setting::table_setting::TableSetting, table::RecordInsert},
    };

    if let Some(session) = Session::find(&auth) {
        return DB.with(|db| {
            match TableSetting::<RecordLabelPreset>::insert(
                db,
                &RecordLabelPreset {
                    id: 0,
                    user_id: session.user_id,
                    name,
                    options,
                },
            ) {
                Ok(record) => Ok(record > 1),
                Err(e) => Err(e)?,
            }
        });
    }

    Err(Error::other("Cannot get user").into())
}

#[post("/api/del_label_preset", auth: crate::backend::auth::SessionId)]
pub async fn del_label_preset(id: i64) -> Result<bool> {
    use std::io::Error;

    use crate::{
        backend::{
            auth::Session, database::db::DB, database::tables::setting::table_setting::TableSetting,
        },
        shared::models::database::setting::types_setting::SettingEmpty,
    };

    if let Some(session) = Session::find(&auth) {
        return DB.with(|db| {
            match TableSetting::<SettingEmpty>::delete_user_setting(db, id, session.user_id) {
                Ok(record) => Ok(record > 1),
                Err(e) => Err(e)?,
            }
        });
    }

    Err(Error::other("Cannot get user").into())
}
