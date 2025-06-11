//! This crate contains all shared UI for the workspace.

use dioxus::prelude::*;

mod dashboard;
pub use dashboard::Dashboard;

mod home;
pub use home::Home;

pub const MAIN_CSS: Asset = asset!("assets/styling/main.css");
