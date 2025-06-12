use dioxus::{
    desktop::{Config, WindowBuilder},
    prelude::*,
};
use dioxus_desktop::{tao::platform::macos::WindowBuilderExtMacOS, LogicalSize};
use dioxus_router::prelude::*;

mod dashboard;
use dashboard::Dashboard;

mod home;
use home::Home;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/dashboard")]
    Dashboard {},
}

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title("Aokiji")
                    .with_max_inner_size(LogicalSize::new(505.0, 900.0))
                    .with_min_inner_size(LogicalSize::new(505.0, 400.0))
                    .with_resizable(true)
                    .with_decorations(true)
                    .with_has_shadow(true)
                    .with_always_on_top(false)
                    .with_transparent(false),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route>{}
    }
}
