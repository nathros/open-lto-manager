use dioxus::prelude::*;

use crate::frontend::components::navbar::Navbar;
use crate::frontend::pages::home::Home;
use crate::frontend::pages::show::Show;
use crate::frontend::pages::test::Test;

use crate::frontend::pages::dbg::db_man::DBMan;
use crate::frontend::pages::dbg::db_tape::DBTape;
use crate::frontend::pages::dbg::db_type::DBType;
use crate::frontend::pages::tape::Tape;

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

    // Debug only
    #[route("/db-man")]
    DBMan {},
    #[route("/db-type")]
    DBType {},
    #[route("/db-tape")]
    DBTape {},
}
