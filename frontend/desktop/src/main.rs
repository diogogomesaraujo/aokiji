use dioxus::{
    desktop::{Config, WindowBuilder},
    prelude::*,
};
use dioxus_desktop::{tao::platform::macos::WindowBuilderExtMacOS, LogicalSize};
use ui::Home;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

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
            Home{}
    }
}
