//! This file initializes the application and contains shared data structures and constants.

use dioxus::{
    desktop::{Config, WindowBuilder},
    prelude::*,
};
use dioxus_desktop::{tao::platform::macos::WindowBuilderExtMacOS, LogicalSize};
use dioxus_router::prelude::*;
use dirs::config_dir;
use frost_sig::{client::ConfigFile, FrostState};
use std::{env::current_dir, error::Error, fs::create_dir_all, path::PathBuf};

mod dashboard;
use dashboard::Dashboard;

mod home;
use home::Home;

/// Asset that represents the path to the app's css file.
const APP_CSS: Asset = asset!("/assets/app.css");

/// Asset that represents the path to the main css file of the application.
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

/// Asset that represents the path to the font's css file (in this case Satoshi).
const SATOSHI_CSS: Asset = asset!("assets/satoshi.css");

/// Constant value of the port used for socket connections.
pub const PORT: u32 = 6705;

/// Enum that represents the different routes of the application.
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    /// Home page of the application
    #[route("/")]
    Home {},

    /// Dashboard of the application
    #[route("/dashboard")]
    Dashboard {},
}

/// Enum that represents the real-time state of an operation.
#[derive(Clone)]
pub enum TransactionState {
    Idle,
    Processing,
    Successful,
    Error(String),
}

/// Struct that represents unites the variables shared across the application.
#[derive(Clone, Debug)]
pub struct AppState {
    /// Path of the account's file.
    pub account_path: String,

    /// Nano account currently open.
    pub nano_account: String,

    /// Public share that identifies the user inside the group.
    pub public_share: String,

    /// FROST parameters of the account.
    pub frost_state: FrostState,

    /// Configuration file.
    pub config_file: ConfigFile,

    /// Configuration file path.
    pub config_file_path: String,
}

impl Default for AppState {
    /// Function that returns a default value for the `AppState`.
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

/// Function that gets or creates the config file directory according to the operating system.
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

/// Main function of the application.
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

/// Default app component that will define the font, layout and background but also initialize the app state.
#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(AppState::default()));

    rsx! {
        document::Link { rel: "stylesheet", href: APP_CSS }
        document::Link { rel: "stylesheet", href: SATOSHI_CSS }
        Router::<Route>{}
    }
}
