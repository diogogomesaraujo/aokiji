//! This file contains the different building blocks that form the dashboard of the Nano shared acconut.

use crate::{AppState, TransactionState, MAIN_CSS, PORT};
use arboard::Clipboard;
use dioxus::prelude::*;
use dioxus_material_icons::{MaterialIcon, MaterialIconStylesheet};
use frost_sig::{
    client::SignInput,
    nano::{
        rpc::{AccountBalance, AccountHistory, AccountInfo, RPCState},
        sign::{Subtype, UnsignedBlock},
    },
};
use routes::{get_nano_price_euro, NanoPriceEuro, NanoPriceResponse};
use std::time::Duration;

/// Constant value for the path of the avatar image.
const AVATAR: Asset = asset!("/assets/images/avatar.png");

/// Function that represents the Nano account's dashboard.
#[component]
pub fn Dashboard() -> Element {
    // mutable state that represents the section that is currently selected.
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
                        PublicShare{}
                        div { style: "display: inline-block; margin-bottom: 14px;" }
                        AccountInfoSection {  }
                    }
                },
                "transaction" => {
                    rsx! {
                        StartTransaction{}
                        div { style: "display: inline-block; margin-bottom: 14px;" }
                        JoinTransaction{}
                        div { style: "display: inline-block; margin-bottom: 14px;" }
                        TransactionConfig{}
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

/// Function that represents the section with the account's balance and receivable Nano.
#[component]
fn Balance() -> Element {
    // represents the shared state of the application
    let app_state = use_context::<Signal<AppState>>();

    // nano account stored in the app state
    let account = app_state.read().nano_account.clone();

    // closure that gets the balance from the RPC
    let balance_future = use_resource(move || {
        let account = account.clone();
        let config = app_state.read().config_file.clone();
        let state = RPCState::new(&config.url);
        async move { AccountBalance::get_from_rpc(&state, &account, &config.key).await }
    });
    let balance_info: AccountBalance = match &*balance_future.read_unchecked() {
        Some(res) => match res {
            Ok(b) => (*b).clone(),
            Err(_) => AccountBalance::default(),
        },
        None => AccountBalance::default(),
    };

    // closure that gets the updated nano price
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
    let receivable_nano = match balance_info.receivable_nano.parse::<f32>() {
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
            strong { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "TOTAL BALANCE" }
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
                        span { id: "secondary", "RECEIVABLE" }
                    }
                }
                div {
                    id: "container",
                    div {
                        id: "fill-card",
                        span { style: "display: inline-block; padding-right: 10px; align-items: center;", "XNO" }
                        strong { id: "sub-heading" , {receivable_nano.to_string()} }
                    }
                }
            }
        }
    }
}

/// Function that represents the header section of the dashboard.
#[component]
fn Header() -> Element {
    // represents the shared state of the application
    let app_state = use_context::<Signal<AppState>>();

    // closure that copies the nano account to the clipboard of the user
    let copy_to_clipboard = move |_| {
        let mut clipboard = match Clipboard::new() {
            Ok(clipboard) => clipboard,
            Err(_) => return,
        };

        // nano account stored in the app state
        let account = app_state.read().nano_account.clone();

        match clipboard.set_text(account) {
            Ok(_) => {}
            Err(_) => return,
        }
    };

    rsx! {
        div {
            id: "header",
            div {
                style: "display: flex; align-items: center; gap: 12px;",
                div {
                    class: "avatar",
                    img {
                        src: "{AVATAR}",
                        style: "width: 100%; height: 100%; object-fit: cover;"
                    }
                }
                div {
                    style: "display: flex; flex-direction: column;",
                    div {
                        style: "display: flex; align-items: center; gap: 0px;",
                        a {
                            class: "nano-account",
                            { app_state.read().nano_account.clone() }
                        }
                        button {
                            class: "clipboard",
                            onclick: copy_to_clipboard,
                            style: "font-size: 20px; margin-left: -8px;",
                            MaterialIcon { name: "content_copy" }
                        }
                    }
                    div { id:"secondary", a { {
                        // FROST parameters stored in the app state
                        let frost_state = app_state.read().frost_state.clone();
                        format!("{} Participants", frost_state.participants)
                    } } }
                }
            }
        }
    }
}

/// Function that represents the Start Transaction section of the account's dashboard.
#[component]
fn StartTransaction() -> Element {
    // mutable state that represents the type of transaction selected
    let mut transaction_type = use_signal(|| "SEND".to_string());

    // mutable state that represents the account that will receive the sent Nano
    let mut receivers_account = use_signal(|| "".to_string());

    // mutable state that represents the amount of Nano sent
    let mut amount = use_signal(|| "0".to_string());

    // mutable synchronous state that represents the state of the transaction in real-time
    let mut transaction_state = use_signal_sync(|| TransactionState::Idle);

    // represents the shared state of the application
    let app_state = use_context::<Signal<AppState>>();

    // closure that opens the socket that will be used for the transaction and also connects as a client
    let open_socket_and_connect = move |_| {
        use_future(move || async move {
            let url = app_state.read().config_file.url.clone();
            let state = RPCState::new(&url);
            let account = app_state.read().nano_account.clone();
            let path = app_state.read().account_path.clone();
            let config = app_state.read().config_file.clone();

            // building the unsigned block according to the type of transaction
            let unsigned_block = match transaction_type.read().as_str() {
                "OPEN" => match UnsignedBlock::create_open(&state, &account, &config.key).await {
                    Ok(block) => block,
                    Err(_) => {
                        transaction_state.set(TransactionState::Error(
                            "Couldn't create the block.".to_string(),
                        ));
                        return;
                    }
                },
                "RECEIVE" => {
                    match UnsignedBlock::create_receive(&state, &account, &config.key).await {
                        Ok(block) => block,
                        Err(_) => {
                            transaction_state.set(TransactionState::Error(
                                "Couldn't create the block.".to_string(),
                            ));
                            return;
                        }
                    }
                }
                _ => match UnsignedBlock::create_send(
                    &state,
                    &account,
                    &receivers_account.read(),
                    &amount.read().parse::<f64>().unwrap_or(0.),
                    &config.key,
                )
                .await
                {
                    Ok(block) => block,
                    Err(_) => {
                        transaction_state.set(TransactionState::Error(
                            "Couldn't create the block.".to_string(),
                        ));
                        return;
                    }
                },
            };

            // getting the sign input from the path stored in the app state
            let mut sign_input = match SignInput::from_file(&path).await {
                Ok(input) => input,
                Err(e) => {
                    transaction_state.set(TransactionState::Error(e.to_string()));
                    return;
                }
            };
            sign_input.subtype = match transaction_type.read().as_str() {
                "RECEIVE" => Subtype::RECEIVE,
                "OPEN" => Subtype::OPEN,
                _ => Subtype::SEND,
            };
            sign_input.message = unsigned_block;
            match sign_input.to_file(&path).await {
                Ok(_) => {}
                Err(e) => {
                    transaction_state.set(TransactionState::Error(e.to_string()));
                    return;
                }
            };

            // notify the user of the state of the transaction
            transaction_state.set(TransactionState::Processing);

            // get the frost state from the shared app state
            let state = app_state.read().frost_state.clone();

            // get the path of the sign input file from the shared app state
            let path = app_state.read().account_path.clone();

            // get the path of the config file from the shared app state
            let config_file_path = app_state.read().config_file_path.clone();

            // open the socket with the correct parameters
            let server = tokio::spawn(async move {
                match frost_sig::server::sign_server::run(
                    "localhost",
                    PORT,
                    state.participants,
                    state.threshold,
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        transaction_state.set(TransactionState::Error(e.to_string()));
                        return;
                    }
                };
            });

            // connect to the socket that was opened
            let client = tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(2)).await;
                match frost_sig::client::sign_client::run(
                    "localhost",
                    PORT,
                    &path,
                    &config_file_path,
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        transaction_state.set(TransactionState::Error(e.to_string()));
                        return;
                    }
                };
            });

            // after processing the transaction notify user
            tokio::spawn(async move {
                let _ = tokio::join!(server, client);
                transaction_state.set(TransactionState::Successful);
            });
        });
    };

    rsx! {
        div {
            id: "card",
            strong { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "START TRANSACTION" }
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
                                value: "0",
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
            {
                match *transaction_state.read() {
                    TransactionState::Processing => {
                        rsx! {
                            div { style: "display: inline-block; margin-bottom: 14px;" }
                            span { id: "secondary", "Processing the transaction..." }
                        }
                    }
                    TransactionState::Successful => {
                        rsx! {
                            div { style: "display: inline-block; margin-bottom: 14px;" }
                            span { id: "secondary", "Transaction was successful." }
                        }
                    }
                    TransactionState::Error(ref e) => {
                        rsx! {
                            div { style: "display: inline-block; margin-bottom: 14px;" }
                            span { id: "secondary", "{e}" }
                        }
                    }
                    _ => {
                        rsx!{}
                    }
                }
            }
            div { style: "display: inline-block; margin-bottom: 36px;" }
            div {
                id: "column-section",
                button {
                    id: "button",
                    disabled: match *transaction_state.read() {
                        TransactionState::Idle | TransactionState::Error(_) => match (receivers_account().as_str(), transaction_type().as_str()) {
                            ("", "SEND") => true,
                            _ => false
                        },
                        _ => true,
                    },
                    onclick: open_socket_and_connect,
                    "Start",
                }
            }
        }
    }
}

#[component]
fn JoinTransaction() -> Element {
    let mut transaction_type = use_signal(|| "SEND".to_string());
    let mut receivers_account = use_signal(|| "".to_string());
    let mut ip_address = use_signal(|| "".to_string());
    let mut amount = use_signal(|| "0".to_string());

    let mut transaction_state = use_signal_sync(|| TransactionState::Idle);

    let app_state = use_context::<Signal<AppState>>();

    let connect_to_socket = move |_| {
        use_future(move || async move {
            let url = app_state.read().config_file.url.clone();
            let state = RPCState::new(&url);
            let account = app_state.read().nano_account.clone();
            let path = app_state.read().account_path.clone();
            let config = app_state.read().config_file.clone();
            let unsigned_block = match transaction_type.read().as_str() {
                "OPEN" => match UnsignedBlock::create_open(&state, &account, &config.key).await {
                    Ok(block) => block,
                    Err(_) => {
                        transaction_state.set(TransactionState::Error(
                            "Couldn't create the block.".to_string(),
                        ));
                        return;
                    }
                },
                "RECEIVE" => {
                    match UnsignedBlock::create_receive(&state, &account, &config.key).await {
                        Ok(block) => block,
                        Err(_) => {
                            transaction_state.set(TransactionState::Error(
                                "Couldn't create the block.".to_string(),
                            ));
                            return;
                        }
                    }
                }
                _ => match UnsignedBlock::create_send(
                    &state,
                    &account,
                    &receivers_account.read(),
                    &amount.read().parse::<f64>().unwrap_or(0.),
                    &config.key,
                )
                .await
                {
                    Ok(block) => block,
                    Err(_) => {
                        transaction_state.set(TransactionState::Error(
                            "Couldn't create the block.".to_string(),
                        ));
                        return;
                    }
                },
            };
            let mut sign_input = match SignInput::from_file(&path).await {
                Ok(input) => input,
                Err(e) => {
                    transaction_state.set(TransactionState::Error(e.to_string()));
                    return;
                }
            };
            sign_input.subtype = match transaction_type.read().as_str() {
                "RECEIVE" => Subtype::RECEIVE,
                "OPEN" => Subtype::OPEN,
                _ => Subtype::SEND,
            };
            sign_input.message = unsigned_block;
            match sign_input.to_file(&path).await {
                Ok(_) => {}
                Err(e) => {
                    transaction_state.set(TransactionState::Error(e.to_string()));
                    return;
                }
            };

            transaction_state.set(TransactionState::Processing);

            let path = app_state.read().account_path.clone();
            let config_file_path = app_state.read().config_file_path.clone();
            let ip_address = ip_address.read().clone();

            let client = tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(2)).await;
                match frost_sig::client::sign_client::run(
                    &ip_address,
                    PORT,
                    &path,
                    &config_file_path,
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        transaction_state.set(TransactionState::Error(e.to_string()));
                        return;
                    }
                };
            });

            tokio::spawn(async move {
                match client.await {
                    Ok(_) => {
                        if !matches!(*transaction_state.read(), TransactionState::Error(_)) {
                            transaction_state.set(TransactionState::Successful);
                        }
                    }
                    Err(e) => {
                        transaction_state.set(TransactionState::Error(format!("{e}")));
                    }
                }
            });

            println!("Client listening.");
        });
    };

    rsx! {
        div {
            id: "card",
            strong { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "JOIN TRANSACTION" }
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
                                value: "0",
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
            {
                match *transaction_state.read() {
                    TransactionState::Processing => {
                        rsx! {
                            div { style: "display: inline-block; margin-bottom: 14px;" }
                            span { id: "secondary", "Processing the transaction..." }
                        }
                    }
                    TransactionState::Successful => {
                        rsx! {
                            div { style: "display: inline-block; margin-bottom: 14px;" }
                            span { id: "secondary", "Transaction was successful." }
                        }
                    }
                    TransactionState::Error(ref e) => {
                        rsx! {
                            div { style: "display: inline-block; margin-bottom: 14px;" }
                            span { id: "secondary", "{e}" }
                        }
                    }
                    _ => {
                        rsx!{}
                    }
                }
            }
            div { style: "display: inline-block; margin-bottom: 36px;" }
            div {
                id: "column-section",
                button {
                    id: "secondary-button",
                    disabled: match *transaction_state.read() {
                        TransactionState::Idle | TransactionState::Error(_) => match (receivers_account().as_str(), ip_address().as_str(), transaction_type().as_str()) {
                            ("", _, "SEND") => true,
                            (_, "", "SEND") => true,
                            _ => false
                        },
                        _ => true,
                    },
                    onclick: connect_to_socket,
                    "Join",
                }
            }
        }
    }
}

#[component]
fn SendIcon() -> Element {
    rsx! {
        div {
            style: "font-size: 40px; transform: scaleX(-1); color: #5B87CF;",
            MaterialIcon { name: "reply" }
        }
    }
}

#[component]
fn ReceiveIcon() -> Element {
    rsx! {
        div {
            style: "font-size: 40px; color: #B7D0FE;",
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
    let transactions = use_resource(async move || {
        let app_state = use_context::<Signal<AppState>>();
        let config = app_state.read().config_file.clone();
        let state = RPCState::new(&config.url);
        let nano_account = app_state.read().nano_account.clone();

        match AccountHistory::get_from_rpc(&state, &nano_account, 50u32, &config.key).await {
            Ok(account_history) => account_history.history,
            Err(_) => Vec::new(),
        }
    });

    rsx! {
        div {
            id: "card",
            div {
                id: "column-section",
                match &*transactions.read_unchecked() {
                    Some(transaction_list) => {
                        rsx! {
                            strong { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "TRANSACTION HISTORY" }
                            for (i, transaction) in transaction_list.iter().enumerate() {
                                div {
                                    id: "transaction",
                                    div {
                                        style: "display: flex; align-items: center; gap: 12px;",
                                        if transaction.r#type.as_str() == "send" {
                                            SendIcon{}
                                        } else {
                                            ReceiveIcon{}
                                        }
                                        div {
                                            style: "flex: 1;",
                                            div {
                                                id: "fill-card",
                                                span { id: "sub-heading" , style: "text-overflow: ellipsis;
                                                  max-width: 200px; white-space: nowrap;
                                                    overflow: hidden;", strong { {format!("{}", transaction.account)} } }
                                                span { id: "secondary" , "XNO" }
                                            }
                                            div {
                                                id: "fill-card",
                                                span { id: "secondary" , style: "text-overflow: ellipsis;
                                                  max-width: 200px; white-space: nowrap;
                                                    overflow: hidden;", strong {  {format!("{}", transaction.hash)} } }
                                                strong { id: "sub-heading" , {format!("{}", transaction.amount.parse::<u128>().unwrap_or(0u128) as f64 / 1_000_000_000_000_000_000_000_000_000_000.0)} }
                                            }
                                        }
                                    }
                                }
                                if i != transaction_list.len() - 1 {
                                    div { style: "display: inline-block; margin-bottom: 14px;" }
                                }
                            }
                        }
                    }
                    None => rsx! { span { id: "secondary", "Loading transactions..." } }
                }
            }
        }
    }
}

#[component]
fn PublicShare() -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let public_share = app_state.read().public_share.clone();
    rsx! {
        div {
            id: "card",
            strong { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "PUBLIC KEY SHARE" }
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
            let config = app_state.read().config_file.clone();
            let state = RPCState::new(&config.url);
            AccountInfo::get_from_rpc(&state, &account, &config.key).await
        }
    });

    match &*account_info_future.read_unchecked() {
        Some(Ok(account_info)) => {
            rsx! {
                div {
                    id: "card",
                    strong { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "FRONTIER" }
                    div {
                        div {
                            id: "fill-card",
                            span { id: "sub-heading" , style: "text-overflow: ellipsis;
                              max-width: 390px; white-space: nowrap;
                                overflow: hidden;", span { {account_info.frontier.clone()} } }
                        }
                    }
                    div { style: "display: inline-block; margin-bottom: 28px;" }
                    strong { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "OPEN BLOCK" }
                    div {
                        div {
                            id: "fill-card",
                            span { id: "sub-heading" , style: "text-overflow: ellipsis;
                              max-width: 390px; white-space: nowrap;
                                overflow: hidden;", span { {account_info.open_block.clone()} } }
                        }
                    }
                    div { style: "display: inline-block; margin-bottom: 28px;" }
                    strong { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "REPRESENTATIVE" }
                    div {
                        div {
                            id: "fill-card",
                            span { id: "sub-heading" , style: "text-overflow: ellipsis;
                              max-width: 390px; white-space: nowrap;
                                overflow: hidden;", span { {account_info.representative.clone()} } }
                        }
                    }
                    div { style: "display: inline-block; margin-bottom: 28px;" }
                    strong { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "BALANCE" }
                    div {
                        div {
                            id: "fill-card",
                            span { id: "sub-heading" , style: "text-overflow: ellipsis;
                              max-width: 390px; white-space: nowrap;
                                overflow: hidden;", span { {account_info.balance.clone()} } }
                        }
                    }
                    div { style: "display: inline-block; margin-bottom: 28px;" }
                    strong { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "BLOCK COUNT" }
                    div {
                        div {
                            id: "fill-card",
                            span { id: "sub-heading" , style: "text-overflow: ellipsis;
                              max-width: 390px; white-space: nowrap;
                                overflow: hidden;", span { {account_info.block_count.clone()} } }
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
fn TransactionConfig() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    let config = app_state.read().config_file.clone();

    let mut rpc_url = use_signal(|| config.url.clone());
    let mut api_key = use_signal(|| config.key.clone());

    let save_config = move |_| {
        let mut config = app_state.read().config_file.clone();
        let config_file_path = app_state.read().config_file_path.clone();
        config.key = api_key.read().clone();
        config.url = rpc_url.read().clone();

        config.to_file_sync(&config_file_path).unwrap();
        app_state.write().config_file = config;
    };

    rsx! {
        div {
            id: "card",
            strong { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "RPC CONFIGURATION" }
            div {
                id: "column-section",
                span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Key:" }
                input {
                    id: "input",
                    value: api_key(),
                    onchange: move |event| api_key.set(event.value()),

                }
            }
            div { style: "display: inline-block; margin-bottom: 14px;" }
            div {
                id: "column-section",
                span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Url:" }
                input {
                    id: "input",
                    value: rpc_url(),
                    onchange: move |event| rpc_url.set(event.value()),

                }
            }
            div { style: "display: inline-block; margin-bottom: 36px;" }
            div {
                id: "column-section",
                button {
                    id: "button",
                    onclick: save_config,
                    "Save",
                }
            }
        }
    }
}
