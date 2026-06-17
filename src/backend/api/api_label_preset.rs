use dioxus::prelude::*;

use crate::shared::models::database::label_preset::model_label_preset::RecordLabelPreset;

#[get("/api/label_preset", auth: crate::backend::auth::SessionId)]
pub async fn get_user_preset() -> Result<Vec<RecordLabelPreset>> {
    use std::io::Error;

    use crate::backend::{
        auth::Session,
        database::{db::DB, tables::label_preset::table_label_preset::TableLabelPreset},
    };

    if let Some(session) = Session::find(&auth) {
        return DB.with(
            |db| match TableLabelPreset::get_user_presets(db, session.user_id) {
                Ok(records) => Ok(records),
                Err(e) => Err(e)?,
            },
        );
    }

    Err(Error::other("Cannot get user").into())
}
