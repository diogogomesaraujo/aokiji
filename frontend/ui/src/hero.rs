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

    let account_history_future = use_resource(|| get_account_history(ACCOUNT, 5));
    let account_history = match &*account_history_future.read_unchecked() {
        Some(res) => format!("{:?}", res),
        None => String::new(),
    };

    let account_balance_future = use_resource(|| get_account_info(ACCOUNT));
    let account_balance = match &*account_balance_future.read_unchecked() {
        Some(res) => format!("{:?}", res),
        None => String::new(),
    };

    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }

        ul {
            li { strong {"Version: "} {version} }
            li { strong {"Account Info: "} {account_info} }
            li { strong {"Account History: "} {account_history} }
            li { strong {"Account Balance: "} {account_balance} }
        }
    }
}
