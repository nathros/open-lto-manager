use dioxus::prelude::*;

use crate::frontend::components::navbar::Navbar;
use crate::frontend::pages::add_job::AddJob;
use crate::frontend::pages::dbg::db_app_state::ShowAppState;
use crate::frontend::pages::dbg::db_file::DBFile;
use crate::frontend::pages::dbg::db_job::DBJob;
use crate::frontend::pages::dbg::db_man::DBMan;
use crate::frontend::pages::dbg::db_tape::DBTape;
use crate::frontend::pages::dbg::db_type::DBType;
use crate::frontend::pages::dbg::db_user::DBUser;
use crate::frontend::pages::home::Home;
use crate::frontend::pages::show::Show;
use crate::frontend::pages::tape::Tape;
use crate::frontend::pages::test::Test;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
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

    // Debug only
    #[route("/db-man")]
    DBMan {},
    #[route("/db-type")]
    DBType {},
    #[route("/db-user")]
    DBUser {},
    #[route("/db-job")]
    DBJob {},
    #[route("/db-file")]
    DBFile {},
    #[route("/db-tape")]
    DBTape {},
    #[route("/db-state")]
    ShowAppState {},
}
