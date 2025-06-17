use dioxus::{
    desktop::{Config, WindowBuilder},
    prelude::*,
};
use dioxus_desktop::{tao::platform::macos::WindowBuilderExtMacOS, LogicalSize};
use dioxus_router::prelude::*;
use frost_sig::FrostState;

mod dashboard;
use dashboard::Dashboard;

mod home;
use home::Home;

mod alert;
use alert::Alert;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

pub const PORT: u32 = 6705;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/alert")]
    Alert {},
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub account_path: String,
    pub nano_account: String,
    pub public_share: String,
    pub frost_state: FrostState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            account_path: "".to_string(),
            nano_account: "".to_string(),
            frost_state: FrostState::new(0, 0),
            public_share: "".to_string(),
        }
    }
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
                    .with_focused(true)
                    .with_transparent(false),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(AppState::default()));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route>{}
    }
}
