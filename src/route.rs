use dioxus::prelude::*;

use crate::frontend::{
    components::header::header_base::Header,
    pages::{
        admin::sessions::Sessions,
        dbg::{
            db_dev::ShowDev, db_file::DBFile, db_job::DBJob, db_job_metadata::DBJobMetaData,
            db_tape::DBTape, db_type::DBType, db_user::DBUser, show_devices::ShowDevices,
        },
        home::Home,
        job::add_job::AddJob,
        library::{generate_label::GenLabel, tape::Tape, view_library::ViewLibrary},
        login::login_user::LoginUser,
        show::Show,
        system::diagnostics::Diagnostics,
        test::Test,
    },
    sandpit::index::Sandpit,
};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Header)]
    #[route("/")]
    Home {},

    #[route("/show")]
    Show {},

    #[route("/test")]
    Test {},

    #[route("/library/tape/:id")]
    Tape { id: i64 },
    #[route("/library/view")]
    ViewLibrary {},
    #[route("/library/generate/label")]
    GenLabel {},

    #[route("/jobs/")]
    AddJob {},

    #[route("/admin/sessions")]
    Sessions {},

    #[route("/system/diagnostics")]
    Diagnostics {},

    // Debug only
    #[route("/show-dev")]
    ShowDevices {},

    #[route("/sandpit?:name")]
    Sandpit { name: String },

    #[route("/db-type")]
    DBType {},
    #[route("/db-user")]
    DBUser {},
    #[route("/db-job")]
    DBJob {},
    #[route("/db-job-meta")]
    DBJobMetaData {},
    #[route("/db-file")]
    DBFile {},
    #[route("/db-tape")]
    DBTape {},

    #[route("/dev")]
    ShowDev {},
    #[end_layout]
    // No layout
    #[route("/login")]
    LoginUser {},
}
