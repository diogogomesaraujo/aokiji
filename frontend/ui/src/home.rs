use std::time::Duration;

use dioxus::prelude::*;
use dioxus_material_icons::MaterialIconStylesheet;

use crate::MAIN_CSS;

const PORT: u32 = 6705;

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
    let mut participants = use_signal(|| "0".to_string());
    let mut threshold = use_signal(|| "0".to_string());
    let mut path = use_signal(|| "".to_string());
    let mut operation_type = use_signal(|| "OPEN".to_string());
    let mut ip_address = use_signal(|| "127.0.0.1".to_string());

    let mut server_handle = use_signal(|| None::<tokio::task::JoinHandle<()>>);
    let mut client_handle = use_signal(|| None::<tokio::task::JoinHandle<()>>);

    let open_socket = move |_| {
        let participants = participants.read().parse::<u32>().unwrap_or(0);
        let threshold = threshold.read().parse::<u32>().unwrap_or(0);
        let path = path.read().clone();

        let server = tokio::spawn(async move {
            match frost_sig::server::keygen_server::run("127.0.0.1", PORT, participants, threshold)
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
            match frost_sig::client::keygen_client::run("127.0.0.1", PORT, &path).await {
                Ok(_) => {
                    println!("Created the server like wonders!")
                }
                Err(e) => {
                    eprintln!("Server error: {}", e);
                }
            };
        });

        server_handle.set(Some(server));
        server_handle.set(Some(client));
        println!("Server listening.")
    };

    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "CREATE ACCOUNT SESSION" }
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
                        min: "0",
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
                        min: "0",
                        onchange: move |event| threshold.set(event.value()),

                    }
                }
                div { style: "display: inline-block; margin-bottom: 14px;" }
                div {
                    id: "column-section",
                    span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Save to Path:" }
                    input {
                        id: "input",
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
                                    onchange: move |event| ip_address.set(event.value()),
                                }
                            }
                        }
                    }
                    _ => rsx!{}
                }
            div { style: "display: inline-block; margin-bottom: 36px;" }
            div {
                id: "column-section",
                button {
                    id: "button",
                    onclick: open_socket,
                    "Create",
                }
            }
        }
    }
}

#[component]
fn OpenAccount() -> Element {
    let mut path = use_signal(|| "".to_string());
    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "OPEN ACCOUNT" }
                div {
                    id: "column-section",
                    span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Select the File:" }
                    input {
                        id: "input",
                        r#type: "file",
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
                }
            div { style: "display: inline-block; margin-bottom: 36px;" }
            div {
                id: "column-section",
                button {
                    id: "secondary-button",
                    "Open",
                    // value: "{input_text}",
                    // oninput: move |event| input_text.set(event.value())
                }
            }
        }
    }
}
