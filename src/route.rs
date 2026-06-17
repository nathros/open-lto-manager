use dioxus::prelude::*;

use crate::frontend::{
    components::header::Header,
    pages::{
        admin::sessions::Sessions,
        dbg::{
            db_app_state::ShowAppState, db_dev::ShowDev, db_file::DBFile, db_job::DBJob,
            db_job_metadata::DBJobMetaData, db_man::DBMan, db_tape::DBTape, db_type::DBType,
            db_user::DBUser, show_devices::ShowDevices,
        },
        home::Home,
        job::add_job::AddJob,
        login::login_user::LoginUser,
        show::Show,
        tape::Tape,
        test::Test,
    },
    sandpit::{
        index::Sandpit, sandpit_button::SandpitButton, sandpit_message::SandpitMessage,
        sandpit_modal::SandpitModal, sandpit_showcase::SandpitShowcase,
    },
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

    #[route("/tape/:id")]
    Tape { id: i64 },

    #[route("/jobs/")]
    AddJob {},

    #[route("/admin/sessions")]
    Sessions {},

    // Debug only
    #[route("/show-dev")]
    ShowDevices {},

    #[route("/sandpit")]
    Sandpit {},
    #[route("/sandpit/showcase")]
    SandpitShowcase {},
    #[route("/sandpit/button")]
    SandpitButton {},
    #[route("/sandpit/modal")]
    SandpitModal {},
    #[route("/sandpit/message")]
    SandpitMessage {},

    #[route("/db-man")]
    DBMan {},
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
    #[route("/db-state")]
    ShowAppState {},
    #[route("/dev")]
    ShowDev {},
    #[end_layout]
    // No layout
    #[route("/login")]
    LoginUser {},
}
