use dioxus::prelude::*;
use routes::{get_account_balance, AccountBalanceResponse};

const ACCOUNT_CSS: Asset = asset!("assets/styling/account.css");

pub fn Account() -> Element {
    rsx! {
        Balance{}
    }
}

pub fn Balance() -> Element {
    let account = "nano_19kqrk7taqnprmy1hcchpkdcpfqnpm7knwdhn9qafhd7b94s99ofngf5ent1";

    let balance_future = use_resource(|| async { get_account_balance(account).await });
    let balance_info: AccountBalanceResponse = match &*balance_future.read_unchecked() {
        Some(res) => (*res).clone(),
        None => AccountBalanceResponse::new(),
    };

    let balance_nano = match balance_info.balance_nano {
        Some(nano) => nano,
        None => String::from("0.0"),
    };

    let pending_nano = match balance_info.pending_nano {
        Some(nano) => nano,
        None => String::from("0.0"),
    };

    rsx! {
        document::Link { rel: "stylesheet", href: ACCOUNT_CSS }
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "TOTAL BALANCE" }
            div {
                style: "display: inline-block; margin-bottom: 36px;",
                div {
                    id: "fill-card",
                    span { id: "sub-heading" , "XNO" }
                    strong { id: "h1" , {balance_nano.clone()} }
                }
                div {
                    id: "fill-card",
                    span { id: "secondary" , "~EUR" }
                    div {
                        id: "secondary" ,
                        strong { id: "sub-heading" , {balance_nano} {"€"} }
                    }
                }
            }
            div {
                div {
                    id: "container",
                    style: "display: inline-block; margin-bottom: 14px;",
                    div {
                        id: "fill-card",
                        span { id: "secondary", "PENDING" }
                    }
                }
                div {
                    id: "container",
                    div {
                        id: "fill-card",
                        span { style: "display: inline-block; padding-right: 10px; align-items: center;", "XNO" }
                        strong { id: "sub-heading" , {pending_nano} }
                    }
                }
            }
        }
    }
}
