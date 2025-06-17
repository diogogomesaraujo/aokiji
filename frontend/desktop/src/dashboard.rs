use std::time::Duration;

use dioxus::prelude::*;

use dioxus_material_icons::{MaterialIcon, MaterialIconStylesheet};
use frost_sig::{
    client::SignInput,
    nano::{
        rpc::{AccountBalance, AccountInfo, RPCState},
        sign::{Subtype, UnsignedBlock},
    },
};
use routes::{get_nano_price_euro, NanoPriceEuro, NanoPriceResponse};

use crate::{AppState, PORT};

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[component]
pub fn Dashboard() -> Element {
    let mut menu_item = use_signal(|| "account_details".to_string());

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
                        AccountInfoSection {  }
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
    let app_state = use_context::<Signal<AppState>>();

    let account = app_state.read().nano_account.clone();

    let balance_future = use_resource(move || {
        let account = account.clone();
        dotenv::dotenv().ok();
        let state = RPCState::new(&std::env::var("URL").unwrap());
        async move { AccountBalance::get_from_rpc(&state, &account).await }
    });
    let balance_info: AccountBalance = match &*balance_future.read_unchecked() {
        Some(res) => match res {
            Ok(b) => (*b).clone(),
            Err(_) => AccountBalance::default(),
        },
        None => AccountBalance::default(),
    };

    let nano_price_future = use_resource(|| async { get_nano_price_euro().await });
    let nano_price = match &*nano_price_future.read_unchecked() {
        Some(res) => (*res).clone(),
        None => NanoPriceResponse {
            nano: Some(NanoPriceEuro { eur: Some(0.) }),
        },
    };

    let balance_nano = match balance_info.balance_nano.parse::<f32>() {
        Ok(b) => b,
        Err(_) => 0.,
    };
    let pending_nano = match balance_info.pending_nano.parse::<f32>() {
        Ok(b) => b,
        Err(_) => 0.,
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
                        strong { id: "sub-heading" , {pending_nano.to_string()} }
                    }
                }
            }
        }
    }
}

#[component]
fn Header() -> Element {
    let app_state = use_context::<Signal<AppState>>();
    rsx! {
        div {
            id: "header",
            div {
                style: "display: flex; flex-direction: column;",
                a {
                    class: "nano-account",
                    { app_state.read().nano_account.clone() }
                }
                div { id:"secondary", a { {
                    let frost_state = app_state.read().frost_state.clone();
                    format!("{} Participants", frost_state.participants)
                } } }
            }
        }
    }
}

#[component]
fn StartTransaction() -> Element {
    let mut transaction_type = use_signal(|| "SEND".to_string());
    let mut receivers_account = use_signal(|| "".to_string());
    let mut amount = use_signal(|| "0".to_string());

    let mut is_completed = use_signal_sync(|| false);

    let app_state = use_context::<Signal<AppState>>();

    let open_socket_and_connect = move |_| {
        dotenv::dotenv().ok();

        use_future(move || async move {
            let state = RPCState::new(&std::env::var("URL").unwrap());
            let account = app_state.read().nano_account.clone();
            let path = app_state.read().account_path.clone();
            let unsigned_block = match transaction_type.read().as_str() {
                "OPEN" => UnsignedBlock::create_open(&state, &account).await.unwrap(),
                "RECEIVE" => UnsignedBlock::create_receive(&state, &account)
                    .await
                    .unwrap(),
                _ => UnsignedBlock::create_send(
                    &state,
                    &account,
                    &receivers_account.read(),
                    &amount.read().parse::<f64>().unwrap_or(0.),
                )
                .await
                .unwrap(),
            };
            let mut sign_input = SignInput::from_file(&path).await.unwrap();
            sign_input.subtype = match transaction_type.read().as_str() {
                "RECEIVE" => Subtype::RECEIVE,
                "OPEN" => Subtype::OPEN,
                _ => Subtype::SEND,
            };
            sign_input.message = unsigned_block;
            sign_input.to_file(&path).await.unwrap();

            let state = app_state.read().frost_state.clone();
            let path = app_state.read().account_path.clone();
            let server = tokio::spawn(async move {
                match frost_sig::server::sign_server::run(
                    "localhost",
                    PORT,
                    state.participants,
                    state.threshold,
                )
                .await
                {
                    Ok(_) => {
                        println!("Created the server like wonders!")
                    }
                    Err(e) => {
                        eprintln!("Server error: {}", e);
                    }
                };
            });

            let client = tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(2)).await;
                match frost_sig::client::sign_client::run("localhost", PORT, &path).await {
                    Ok(_) => {
                        println!("Created the server like wonders!")
                    }
                    Err(e) => {
                        eprintln!("Server error: {}", e);
                    }
                };
            });

            tokio::spawn(async move {
                let _ = tokio::join!(server, client);
                is_completed.set(true);
            });

            println!("Server and Clients listening.");
        });
    };

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
            {
                match *is_completed.read() {
                    true => {
                        rsx! {
                            span { id: "secondary", "Transaction was successful." }
                            div { style: "display: inline-block; margin-bottom: 36px;" }
                        }
                    }
                    _ => {
                        rsx!{}
                    }
                }
            }
            div {
                id: "column-section",
                button {
                    id: "button",
                    onclick: open_socket_and_connect,
                    "Start",
                    // value: "{input_text}",
                }
            }
        }
    }
}

#[component]
fn JoinTransaction() -> Element {
    let mut transaction_type = use_signal(|| "SEND".to_string());
    let mut ip_address = use_signal(|| "127.0.0.1".to_string());
    let mut receivers_account = use_signal(|| "".to_string());
    let mut amount = use_signal(|| "0".to_string());

    let app_state = use_context::<Signal<AppState>>();

    let write_block_to_file = move |_| {
        dotenv::dotenv().ok();

        use_future(move || async move {
            let state = RPCState::new(&std::env::var("URL").unwrap());
            let account = app_state.read().nano_account.clone();
            let path = app_state.read().account_path.clone();
            let unsigned_block = match transaction_type.read().as_str() {
                "OPEN" => UnsignedBlock::create_open(&state, &account).await.unwrap(),
                "RECEIVE" => UnsignedBlock::create_receive(&state, &account)
                    .await
                    .unwrap(),
                _ => UnsignedBlock::create_send(
                    &state,
                    &account,
                    &receivers_account.read(),
                    &amount.read().parse::<f64>().unwrap_or(0.),
                )
                .await
                .unwrap(),
            };
            let mut sign_input = SignInput::from_file(&path).await.unwrap();
            sign_input.subtype = match transaction_type.read().as_str() {
                "RECEIVE" => Subtype::RECEIVE,
                "OPEN" => Subtype::OPEN,
                _ => Subtype::SEND,
            };
            sign_input.message = unsigned_block;
            sign_input.to_file(&path).await.unwrap();
        });
    };

    let connect_to_socket = move |_| {
        write_block_to_file(());

        let path = app_state.read().account_path.clone();
        let ip_address = ip_address.read().clone();

        let client = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(2)).await;
            match frost_sig::client::sign_client::run(&ip_address, PORT, &path).await {
                Ok(_) => {
                    println!("Created the server like wonders!")
                }
                Err(e) => {
                    eprintln!("Server error: {}", e);
                }
            };
        });

        tokio::spawn(async move {
            let _ = tokio::join!(client);
            //is_completed.set(true);
        });

        println!("Client listening.");
    };

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
                        div { style: "display: inline-block; margin-bottom: 14px;" }
                        div {
                            id: "column-section",
                            span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Receiver's Account:" }
                            input {
                                id: "input",
                                onchange: move |event| receivers_account.set(event.value()),
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
                    onclick: connect_to_socket,
                    "Join",
                    // value: "{input_text}",
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
    let app_state = use_context::<Signal<AppState>>();
    let public_share = app_state.read().public_share.clone();
    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "PUBLIC KEY SHARE" }
            div {
                id: "column-section",
                div {
                    style: "display: flex; align-items: center; gap: 12px;",
                    PersonIconYellow {}
                    div {
                        style: "flex: 1;",
                        div {
                            id: "fill-card",
                            span { id: "sub-heading" , style: "text-overflow: ellipsis;
                              max-width: 340px; white-space: nowrap;
                                overflow: hidden;", strong { {format!("{}", public_share.to_uppercase())} } }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AccountInfoSection() -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let account = app_state.read().nano_account.clone();

    let account_info_future = use_resource(move || {
        let account = account.clone();
        async move {
            dotenv::dotenv().ok();
            let state = RPCState::new(&std::env::var("URL").unwrap());
            AccountInfo::get_from_rpc(&state, &account).await
        }
    });

    match &*account_info_future.read_unchecked() {
        Some(Ok(account_info)) => {
            rsx! {
                div {
                    id: "card",
                    span { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "FRONTIER" }
                    div {
                        div {
                            id: "fill-card",
                            span { id: "sub-heading" , style: "text-overflow: ellipsis;
                              max-width: 390px; white-space: nowrap;
                                overflow: hidden;", span { {account_info.frontier.clone()} } }
                        }
                    }
                    div { style: "display: inline-block; margin-bottom: 28px;" }
                    span { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "OPEN BLOCK" }
                    div {
                        div {
                            id: "fill-card",
                            span { id: "sub-heading" , style: "text-overflow: ellipsis;
                              max-width: 390px; white-space: nowrap;
                                overflow: hidden;", span { {account_info.open_block.clone()} } }
                        }
                    }
                    div { style: "display: inline-block; margin-bottom: 28px;" }
                    span { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "REPRESENTATIVE" }
                    div {
                        div {
                            id: "fill-card",
                            span { id: "sub-heading" , style: "text-overflow: ellipsis;
                              max-width: 390px; white-space: nowrap;
                                overflow: hidden;", span { {account_info.representative_block.clone()} } }
                        }
                    }
                    div { style: "display: inline-block; margin-bottom: 28px;" }
                    span { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "BALANCE" }
                    div {
                        div {
                            id: "fill-card",
                            span { id: "sub-heading" , style: "text-overflow: ellipsis;
                              max-width: 390px; white-space: nowrap;
                                overflow: hidden;", span { {account_info.balance.clone()} } }
                        }
                    }
                }
            }
        }
        Some(Err(_)) => {
            rsx! {
                div {
                    id: "card",
                    span { id: "secondary", "Open your account with a transaction to see more details." }
                }
            }
        }
        None => {
            rsx! {
                div {
                    id: "card",
                    span { id: "secondary", "Loading account information..." }
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
