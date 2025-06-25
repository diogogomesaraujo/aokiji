//! This file contains the different building blocks that form the home page of the application.

use crate::{AppState, Route, TransactionState, MAIN_CSS, PORT};
use dioxus::prelude::*;
use dioxus_material_icons::MaterialIconStylesheet;
use dioxus_router::hooks::use_navigator;
use frost_sig::{client::SignInput, nano::account::public_key_to_nano_account};
use std::{
    fs::File,
    io::{BufReader, Read},
    time::Duration,
};

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        MaterialIconStylesheet{}
        div {
            id: "page",
            CreateAccountSession{}
            div { style: "display: inline-block; margin-bottom: 28px;" }
            OpenAccount{}
        }
    }
}

#[component]
fn CreateAccountSession() -> Element {
    let mut participants = use_signal(|| "2".to_string());
    let mut threshold = use_signal(|| "2".to_string());
    let mut path = use_signal(|| "account.json".to_string());
    let mut operation_type = use_signal(|| "OPEN".to_string());
    let mut ip_address = use_signal(|| "localhost".to_string());
    let mut is_completed = use_signal_sync(|| false);
    let mut transaction_state = use_signal_sync(|| TransactionState::Idle);

    let mut app_state = use_context::<Signal<AppState>>();
    let nav = use_navigator();

    use_effect(move || {
        if is_completed() {
            app_state.write().account_path = path.read().to_string();

            let sign_input: SignInput = {
                match File::open(path.read().to_string()) {
                    Ok(file) => {
                        let mut buf_reader = BufReader::new(file);
                        let mut contents = String::new();
                        buf_reader.read_to_string(&mut contents).unwrap();

                        serde_json::from_str::<SignInput>(&contents).unwrap()
                    }
                    Err(_) => {
                        nav.push(Route::Home {});
                        SignInput::default()
                    }
                }
            };

            app_state.write().nano_account =
                public_key_to_nano_account(&sign_input.public_aggregated_key.to_bytes());
            app_state.write().frost_state = sign_input.state;
            nav.push(Route::Dashboard {});
        }
    });

    let open_and_connect_to_socket = move |_| {
        let participants = participants.read().parse::<u32>().unwrap_or(0);
        let threshold = threshold.read().parse::<u32>().unwrap_or(0);
        let path = path.read().clone();

        let server = tokio::spawn(async move {
            transaction_state.set(TransactionState::Processing);
            match frost_sig::server::keygen_server::run("localhost", PORT, participants, threshold)
                .await
            {
                Ok(_) => {}
                Err(_) => {
                    transaction_state.set(TransactionState::Error(
                        "Failed to process the transaction".to_string(),
                    ));
                }
            };
        });

        let client = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(2)).await;
            match frost_sig::client::keygen_client::run("localhost", PORT, &path).await {
                Ok(_) => {}
                Err(_) => {
                    transaction_state.set(TransactionState::Error(
                        "Failed to process the transaction".to_string(),
                    ));
                }
            };
        });

        tokio::spawn(async move {
            let _ = tokio::join!(server, client);
            is_completed.set(true);
        });

        println!("Server and Clients listening.");
    };

    let connect_to_socket = move |_| {
        let path = path.read().clone();
        let ip_address = ip_address.read().clone();

        let client = tokio::spawn(async move {
            match frost_sig::client::keygen_client::run(&ip_address, PORT, &path).await {
                Ok(_) => {}
                Err(_) => {
                    transaction_state.set(TransactionState::Error(
                        "Failed to process the transaction".to_string(),
                    ));
                }
            };
        });

        tokio::spawn(async move {
            let _ = tokio::join!(client);
            is_completed.set(true);
            transaction_state.set(TransactionState::Successful);
        });
    };

    rsx! {
        div {
            id: "card",
            strong { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "CREATE ACCOUNT SESSION" }
                div { style: "display: inline-block; margin-bottom: 14px;" }
                div {
                    id: "column-section",
                    span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Type:" }
                    select {
                        id: "select",
                        onchange: move |event| operation_type.set(event.value()),
                        option { value: "OPEN", "OPEN" }
                        option { value: "JOIN", "JOIN" }
                    }
                }
                div { style: "display: inline-block; margin-bottom: 14px;" }
                div {
                    id: "column-section",
                    span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Participants:" }
                    input {
                        id: "input",
                        r#type: "number",
                        initial_value: 2,
                        min: 2,
                        onchange: move |event| participants.set(event.value()),

                    }
                }
                div { style: "display: inline-block; margin-bottom: 14px;" }
                div {
                    id: "column-section",
                    span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Threshold:" }
                    input {
                        id: "input",
                        r#type: "number",
                        value: 2,
                        min: 2,
                        max: participants,
                        onchange: move |event| threshold.set(event.value()),

                    }
                }
                div { style: "display: inline-block; margin-bottom: 14px;" }
                div {
                    id: "column-section",
                    span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Save to File:" }
                    input {
                        id: "input",
                        initial_value: path,
                        onchange: move |event| path.set(event.value()),
                    }
                }
                match operation_type.to_string().as_str() {
                    "JOIN" => {
                        rsx!{
                            div { style: "display: inline-block; margin-bottom: 14px;" }
                            div {
                                id: "column-section",
                                span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "IP Address:" }
                                input {
                                    id: "input",
                                    initial_value: ip_address,
                                    onchange: move |event| ip_address.set(event.value()),
                                }
                            }
                            {
                                match *transaction_state.read() {
                                    TransactionState::Processing => {
                                        rsx! {
                                            div { style: "display: inline-block; margin-bottom: 14px;" }
                                            span { id: "secondary", "Creating the account..." }
                                        }
                                    }
                                    TransactionState::Successful => {
                                        rsx! {
                                            div { style: "display: inline-block; margin-bottom: 14px;" }
                                            span { id: "secondary", "Successfully created the account." }
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
                                    onclick: connect_to_socket,
                                    "Join",
                                }
                            }
                        }
                    }
                    _ => rsx!{
                        {
                            match *transaction_state.read() {
                                TransactionState::Processing => {
                                    rsx! {
                                        div { style: "display: inline-block; margin-bottom: 14px;" }
                                        span { id: "secondary", "Creating the account..." }
                                    }
                                }
                                TransactionState::Successful => {
                                    rsx! {
                                        div { style: "display: inline-block; margin-bottom: 14px;" }
                                        span { id: "secondary", "Successfully created the account." }
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
                                disabled: match transaction_state() {
                                    TransactionState::Idle | TransactionState::Error(_) => match path.read().to_string().as_str() {
                                        "" => true,
                                        _ => false
                                    },
                                    _ => true,
                                },
                                onclick: open_and_connect_to_socket,
                                "Create",
                            }
                        }
                    }
                }
        }
    }
}

#[component]
fn OpenAccount() -> Element {
    let mut path = use_signal(|| "".to_string());
    let mut is_processing = use_signal(|| TransactionState::Idle);

    let mut app_state = use_context::<Signal<AppState>>();
    let nav = use_navigator();

    let open_dashboard_with_account = move |_| {
        app_state.write().account_path = path.read().to_string();

        is_processing.set(TransactionState::Processing);

        let sign_input: SignInput = {
            match File::open(path.read().to_string()) {
                Ok(file) => {
                    let mut buf_reader = BufReader::new(file);
                    let mut contents = String::new();
                    match buf_reader.read_to_string(&mut contents) {
                        Ok(_) => {}
                        Err(e) => {
                            is_processing.set(TransactionState::Error(e.to_string()));
                            return;
                        }
                    };
                    is_processing.set(TransactionState::Successful);

                    match serde_json::from_str::<SignInput>(&contents) {
                        Ok(input) => input,
                        Err(_) => {
                            is_processing.set(TransactionState::Error(
                                "File has invalid format.".to_string(),
                            ));
                            return;
                        }
                    }
                }
                Err(_) => {
                    is_processing.set(TransactionState::Error(
                        "Couldn't open the file.".to_string(),
                    ));
                    return;
                }
            }
        };

        app_state.write().nano_account =
            public_key_to_nano_account(&sign_input.public_aggregated_key.to_bytes());
        app_state.write().frost_state = sign_input.state;
        app_state.write().public_share = hex::encode(sign_input.own_public_share.as_bytes());
        nav.push(Route::Dashboard {});
    };

    rsx! {
        div {
            id: "card",
            strong { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "OPEN ACCOUNT" }
            div {
                id: "column-section",
                span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Select the File:" }
                div {
                    style: "display: flex;
                            flex-direction: row;
                            align-items: center;
                            max-width: 100%;
                            overflow: hidden;
                            background-color: #161e26;
                            border-radius: 12px;
                            padding: 8px 12px;",
                    input {
                        id: "input",
                        r#type: "file",
                        style: "color: transparent; width: auto; flex-shrink: 0;",
                        onchange: move |event| {
                            match &event.files() {
                                Some(file_engine) => {
                                    let files = file_engine.files();
                                    match files.get(0) {
                                        Some(file) => {
                                            path.set(file.clone());
                                        },
                                        None => {}
                                    }
                                },
                                None => {}
                            }
                        },
                    }
                    span {
                        id: "secondary",
                        style: "white-space: nowrap;
                                overflow: hidden;
                                text-overflow: ellipsis;
                                max-width: 200px;
                                display: inline-block;
                                margin-left: -220px;",
                        match path().as_str() {
                            "" => "No file selected.",
                            path => path,
                        }
                    }
                }
            }
            {
                match *is_processing.read() {
                    TransactionState::Processing => {
                        rsx! {
                            div { style: "display: inline-block; margin-bottom: 14px;" }
                            span { id: "secondary", "Opening the account..." }
                        }
                    }
                    TransactionState::Successful => {
                        rsx! {
                            div { style: "display: inline-block; margin-bottom: 14px;" }
                            span { id: "secondary", "Successfully opened the account." }
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
                    disabled: match is_processing() {
                        TransactionState::Idle | TransactionState::Error(_) => match path.read().to_string().as_str() {
                            "" => true,
                            _ => false
                        },
                        _ => true,
                    },
                    onclick: open_dashboard_with_account,
                    "Open",
                }
            }
        }
    }
}
