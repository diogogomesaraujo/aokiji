use dioxus::prelude::*;
use routes::*;

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
const ACCOUNT: &str = "nano_19kqrk7taqnprmy1hcchpkdcpfqnpm7knwdhn9qafhd7b94s99ofngf5ent1";

#[component]
pub fn Hero() -> Element {
    let version_future = use_resource(get_version);

    let version = match &*version_future.read_unchecked() {
        Some(res) => format!("{:?}", res),
        None => String::new(),
    };

    let account_info_future = use_resource(|| get_account_info(ACCOUNT));

    let account_info = match &*account_info_future.read_unchecked() {
        Some(res) => format!("{:?}", res),
        None => String::new(),
    };

    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }

        div {
            a { "Version: {version}" }
            a { "Account Info: {account_info}" }
        }
    }
}
