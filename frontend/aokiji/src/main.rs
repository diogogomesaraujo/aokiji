use std::{env::current_dir, error::Error, fs::create_dir_all, path::PathBuf};

use dioxus::{
    desktop::{Config, WindowBuilder},
    prelude::*,
};
use dioxus_desktop::{tao::platform::macos::WindowBuilderExtMacOS, LogicalSize};
use dioxus_router::prelude::*;
use dirs::config_dir;
use frost_sig::{client::ConfigFile, FrostState};

mod dashboard;
use dashboard::Dashboard;

mod home;
use home::Home;

const APP_CSS: Asset = asset!("/assets/app.css");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const SATOSHI_CSS: Asset = asset!("assets/satoshi.css");
pub const PORT: u32 = 6705;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/dashboard")]
    Dashboard {},
}

#[derive(Clone)]
pub enum TransactionState {
    Idle,
    Processing,
    Successful,
    Error(String),
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub account_path: String,
    pub nano_account: String,
    pub public_share: String,
    pub frost_state: FrostState,
    pub config_file: ConfigFile,
    pub config_file_path: String,
}

impl Default for AppState {
    fn default() -> Self {
        let config_file_path = {
            let config_path = get_config_directory().unwrap_or(PathBuf::default());
            config_path
                .join("config.json")
                .into_os_string()
                .into_string()
                .unwrap_or("".to_string())
        };

        Self {
            account_path: "".to_string(),
            nano_account: "".to_string(),
            frost_state: FrostState::new(0, 0),
            public_share: "".to_string(),
            config_file: ConfigFile::from_file_sync(&config_file_path).unwrap_or(ConfigFile::new()),
            config_file_path,
        }
    }
}

fn get_config_directory() -> Result<PathBuf, Box<dyn Error>> {
    let app = "Aokiji";

    let config_dir = match config_dir() {
        Some(config_dir) => config_dir.join(app),
        _ => current_dir()?.join(app),
    };

    if !config_dir.exists() {
        create_dir_all(&config_dir)?;
    }

    Ok(config_dir)
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
        document::Link { rel: "stylesheet", href: APP_CSS }
        document::Link { rel: "stylesheet", href: SATOSHI_CSS }
        Router::<Route>{}
    }
}
