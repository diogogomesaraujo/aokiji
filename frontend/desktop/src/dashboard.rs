use dioxus::prelude::*;

use dioxus_material_icons::{MaterialIcon, MaterialIconStylesheet};
use routes::{
    get_account_balance, get_nano_price_euro, AccountBalanceResponse, NanoPriceEuro,
    NanoPriceResponse,
};

use crate::AppState;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[component]
pub fn Dashboard() -> Element {
    let mut menu_item = use_signal(|| "account_details".to_string());

    let mut app_state = use_context::<Signal<AppState>>();

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        MaterialIconStylesheet{}
        div {
            id: "page",
            Header{}
            div { style: "display: inline-block; margin-bottom: 28px;" }
            div {
                id: "header",
                div {
                    style: "display: flex; flex-direction: row;",
                    button { id: "menu-button", onclick: move |_| menu_item.set("account_details".to_string()), "Account Details" }
                    div { style: "display: inline-block; margin-left: 14px;" }
                    button { id: "menu-button", onclick: move |_| menu_item.set("transaction".to_string()), "Transaction" }
                    div { style: "display: inline-block; margin-left: 14px;" }
                    button { id: "menu-button", onclick: move |_| menu_item.set("history".to_string()), "History" }
                }
            }
            div { style: "display: inline-block; margin-bottom: 14px;" }
            match menu_item.to_string().as_str() {
                "account_details" => {
                    rsx! {
                        Balance{}
                        div { style: "display: inline-block; margin-bottom: 14px;" }
                        Participants{}
                        div { style: "display: inline-block; margin-bottom: 14px;" }
                        AccountInfo{}
                    }
                },
                "transaction" => {
                    rsx! {
                        StartTransaction{}
                        div { style: "display: inline-block; margin-bottom: 14px;" }
                        JoinTransaction{}
                    }
                },
                "history" => {
                    rsx! {
                        Transactions{}
                    }
                },
                _ => rsx!{}
            }
        }
    }
}

#[component]
fn Balance() -> Element {
    let account = "nano_19kqrk7taqnprmy1hcchpkdcpfqnpm7knwdhn9qafhd7b94s99ofngf5ent1";

    let balance_future = use_resource(|| async { get_account_balance(account).await });
    let balance_info: AccountBalanceResponse = match &*balance_future.read_unchecked() {
        Some(res) => (*res).clone(),
        None => AccountBalanceResponse::new(),
    };

    let nano_price_future = use_resource(|| async { get_nano_price_euro().await });
    let nano_price = match &*nano_price_future.read_unchecked() {
        Some(res) => (*res).clone(),
        None => NanoPriceResponse {
            nano: Some(NanoPriceEuro { eur: Some(0.) }),
        },
    };

    let balance_nano = match balance_info.balance_nano {
        Some(nano) => match nano.parse::<f32>() {
            Ok(nano) => nano,
            Err(_) => 0.,
        },
        None => 0.,
    };

    let pending_nano = match balance_info.pending_nano {
        Some(nano) => nano,
        None => String::from("0.0"),
    };

    let nano_price = match nano_price.nano {
        Some(nano) => match nano.eur {
            Some(price) => price,
            None => 0.,
        },
        None => 0.,
    };

    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "TOTAL BALANCE" }
            div {
                style: "display: inline-block; margin-bottom: 36px;",
                div {
                    id: "fill-card",
                    span { id: "sub-heading" , "XNO" }
                    strong { id: "h1" , {balance_nano.clone().to_string()} }
                }
                div {
                    id: "fill-card",
                    span { id: "secondary" , "~EUR" }
                    div {
                        id: "secondary" ,
                        strong { id: "sub-heading" , {format!("{:.2}", nano_price * balance_nano)} {"€"} }
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

#[component]
fn Header() -> Element {
    rsx! {
        div {
            id: "header",
            div {
                style: "display: flex; flex-direction: column;",
                a { id:"h2", style: "font-weight: bold; text-overflow: ellipsis;
                  max-width: 400px; white-space: nowrap;
                    overflow: hidden;", "nano_1smubapuampnxtq14taxt8c9rc5f97hj7e8kqer4u6p94cre5g6qq3yxa4f3" }
                div { id:"secondary", a { "3 Participants" } }
            }
        }
    }
}

#[component]
fn StartTransaction() -> Element {
    let mut transaction_type = use_signal(|| "SEND".to_string());
    let mut receivers_account = use_signal(|| "".to_string());
    let mut amount = use_signal(|| "0".to_string());
    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "START TRANSACTION" }
            div {
                id: "column-section",
                span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Type:" }
                select {
                    id: "select",
                    onchange: move |event| transaction_type.set(event.value()),
                    option { value: "SEND", "SEND" }
                    option { value: "RECEIVE", "RECEIVE" }
                    option { value: "OPEN", "OPEN" }
                }
            }
            match transaction_type.to_string().as_str() {
                "SEND" => {
                    rsx! {
                        div { style: "display: inline-block; margin-bottom: 14px;" }
                        div {
                            id: "column-section",
                            span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Receiver's Account:" }
                            input {
                                id: "input",
                                onchange: move |event| receivers_account.set(event.value()),
                            }
                        }
                        div { style: "display: inline-block; margin-bottom: 14px;" }
                        div {
                            id: "column-section",
                            span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Amount (XNO):" }
                            input {
                                id: "input",
                                r#type: "number",
                                min: "0",
                                onchange: move |event| amount.set(event.value()),

                            }
                        }
                    }
                }
                _ => {
                    rsx! {
                    }
                }
            }
            div { style: "display: inline-block; margin-bottom: 36px;" }
            div {
                id: "column-section",
                button {
                    id: "button",
                    "Start",
                    // value: "{input_text}",
                    // oninput: move |event| input_text.set(event.value())
                }
            }
        }
    }
}

#[component]
fn JoinTransaction() -> Element {
    let mut transaction_type = use_signal(|| "SEND".to_string());
    let mut ip_address = use_signal(|| "127.0.0.1".to_string());
    let mut amount = use_signal(|| "0".to_string());
    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "JOIN TRANSACTION" }
            div {
                id: "column-section",
                span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Type:" }
                select {
                    id: "select",
                    onchange: move |event| transaction_type.set(event.value()),
                    option { value: "SEND", "SEND" }
                    option { value: "RECEIVE", "RECEIVE" }
                    option { value: "OPEN", "OPEN" }
                }
            }
            div { style: "display: inline-block; margin-bottom: 14px;" }
            div {
                id: "column-section",
                span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "IP Address:" }
                input {
                    id: "input",
                    onchange: move |event| ip_address.set(event.value()),
                }
            }
            div { style: "display: inline-block; margin-bottom: 14px;" }
            match transaction_type.to_string().as_str() {
                "SEND" => {
                    rsx! {
                        div {
                            id: "column-section",
                            span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Amount (XNO):" }
                            input {
                                id: "input",
                                r#type: "number",
                                min: "0",
                                onchange: move |event| amount.set(event.value()),

                            }
                        }
                    }
                }
                _ => {
                    rsx!{}
                }
            }
            div { style: "display: inline-block; margin-bottom: 36px;" }
            div {
                id: "column-section",
                button {
                    id: "secondary-button",
                    "Join",
                    // value: "{input_text}",
                    // oninput: move |event| input_text.set(event.value())
                }
            }
        }
    }
}

#[component]
fn SendIcon() -> Element {
    rsx! {
        div {
            style: "font-size: 40px; transform: scaleX(-1); color: #B7D0FE;",
            MaterialIcon { name: "reply" }
        }
    }
}

#[component]
fn ReceiveIcon() -> Element {
    rsx! {
        div {
            style: "font-size: 40px; color: #5B87CF;",
            MaterialIcon { name: "reply" }
        }
    }
}

#[component]
fn PersonIconBlue() -> Element {
    rsx! {
        div {
            style: "font-size: 40px; transform: scaleX(-1); color: #5B87CF;",
            MaterialIcon { name: "person" }
        }
    }
}

#[component]
fn PersonIconYellow() -> Element {
    rsx! {
        div {
            style: "font-size: 40px; transform: scaleX(-1); color: #B7D0FE;",
            MaterialIcon { name: "person" }
        }
    }
}

#[component]
fn Transactions() -> Element {
    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "TRANSACTION HISTORY" }
            div {
                id: "column-section",
                div {
                    id: "transaction",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        SendIcon{}
                        div {
                            style: "flex: 1;",
                            div {
                                id: "fill-card",
                                span { id: "sub-heading" , style: "text-overflow: ellipsis;
                                  max-width: 200px; white-space: nowrap;
                                    overflow: hidden;", strong { "nano_1smubapuampnxtq14taxt8c9rc5f97hj7e8kqer4u6p94cre5g6qq3yxa4f3" } }
                                span { id: "secondary" , "XNO" }
                            }
                            div {
                                id: "fill-card",
                                span { id: "secondary" , style: "text-overflow: ellipsis;
                                  max-width: 200px; white-space: nowrap;
                                    overflow: hidden;", strong { "E3C52113AABA834B59B7BF4C27CBF5DBDDF0E23D5157AFBA93BC845D1B3C3487" } }
                                strong { id: "sub-heading" , "2.21353" }
                            }
                        }
                    }
                }
                div { style: "display: inline-block; margin-bottom: 14px;" }
                div {
                    id: "transaction",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        ReceiveIcon{}
                        div {
                            style: "flex: 1;",
                            div {
                                id: "fill-card",
                                span { id: "sub-heading" , style: "text-overflow: ellipsis;
                                  max-width: 200px; white-space: nowrap;
                                    overflow: hidden;", strong { "nano_1smubapuampnxtq14taxt8c9rc5f97hj7e8kqer4u6p94cre5g6qq3yxa4f3" } }
                                span { id: "secondary" , "XNO" }
                            }
                            div {
                                id: "fill-card",
                                span { id: "secondary" , style: "text-overflow: ellipsis;
                                  max-width: 200px; white-space: nowrap;
                                    overflow: hidden;", strong { "E3C52113AABA834B59B7BF4C27CBF5DBDDF0E23D5157AFBA93BC845D1B3C3487" } }
                                strong { id: "sub-heading" , "2.21353" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Participants() -> Element {
    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "PARTICIPANTS" }
            div {
                id: "column-section",
                div {
                    id: "transaction",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        PersonIconYellow{}
                        div {
                            style: "flex: 1;",
                            div {
                                id: "fill-card",
                                span { id: "sub-heading" , style: "text-overflow: ellipsis;
                                  max-width: 340px; white-space: nowrap;
                                    overflow: hidden;", strong { "E3C52113AABA834B59B7BF4C27CBF5DBDDF0E23D5157AFBA93BC845D1B3C3487" } }
                            }
                        }
                    }
                }
                div { style: "display: inline-block; margin-bottom: 14px;" }
                div {
                    id: "transaction",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        PersonIconBlue{}
                        div {
                            style: "flex: 1;",
                            div {
                                id: "fill-card",
                                span { id: "sub-heading" , style: "text-overflow: ellipsis;
                                  max-width: 340px; white-space: nowrap;
                                    overflow: hidden;", strong { "E3C52113AABA834B59B7BF4C27CBF5DBDDF0E23D5157AFBA93BC845D1B3C3487" } }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AccountInfo() -> Element {
    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "FRONTIER" }
            div {
                div {
                    id: "fill-card",
                    span { id: "sub-heading" , style: "text-overflow: ellipsis;
                      max-width: 390px; white-space: nowrap;
                        overflow: hidden;", span { "E3C52113AABA834B59B7BF4C27CBF5DBDDF0E23D5157AFBA93BC845D1B3C3487" } }
                }
            }
            div { style: "display: inline-block; margin-bottom: 28px;" }
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "OPEN BLOCK" }
            div {
                div {
                    id: "fill-card",
                    span { id: "sub-heading" , style: "text-overflow: ellipsis;
                      max-width: 390px; white-space: nowrap;
                        overflow: hidden;", span { "E3C52113AABA834B59B7BF4C27CBF5DBDDF0E23D5157AFBA93BC845D1B3C3487" } }
                }
            }
            div { style: "display: inline-block; margin-bottom: 28px;" }
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "REPRESENTATIVE" }
            div {
                div {
                    id: "fill-card",
                    span { id: "sub-heading" , style: "text-overflow: ellipsis;
                      max-width: 390px; white-space: nowrap;
                        overflow: hidden;", span { "nano_1zuksmn4e8tjw1ch8m8fbrwy5459bx8645o9euj699rs13qy6ysjhrewioey" } }
                }
            }
            div { style: "display: inline-block; margin-bottom: 28px;" }
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "BALANCE" }
            div {
                div {
                    id: "fill-card",
                    span { id: "sub-heading" , style: "text-overflow: ellipsis;
                      max-width: 390px; white-space: nowrap;
                        overflow: hidden;", span { "16756113036167018697960000000000" } }
                }
            }
            div { style: "display: inline-block; margin-bottom: 28px;" }
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "BLOCK COUNT" }
            div {
                div {
                    id: "fill-card",
                    span { id: "sub-heading" , style: "text-overflow: ellipsis;
                      max-width: 390px; white-space: nowrap;
                        overflow: hidden;", span { "202" } }
                }
            }
        }
    }
}

#[component]
fn Wallets() -> Element {
    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "SHARED WALLETS" }
            div {
                id: "column-section",
                div {
                    id: "transaction",
                    div {
                        div {
                            id: "fill-card",
                            span { id: "sub-heading" , strong { "Evil Corp." } }
                            span { id: "secondary" , "XNO" }
                        }
                        div {
                            id: "fill-card",
                                span { id: "secondary" , "5 Participants" }
                            strong { id: "sub-heading" , "13482.543" }
                        }
                    }
                }
                div { style: "display: inline-block; margin-bottom: 14px;" }
                div {
                    id: "transaction",
                    div {
                        div {
                            id: "fill-card",
                            span { id: "sub-heading" , strong { "Shelby Company Ltd." } }
                            span { id: "secondary" , "XNO" }
                        }                        div {
                            id: "fill-card",
                                span { id: "secondary" ,  "3 Participants" }
                            strong { id: "sub-heading" , "2.21353" }
                        }
                    }
                }
            }
        }
    }
}
