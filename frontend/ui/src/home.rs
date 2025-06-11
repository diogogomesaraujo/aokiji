use dioxus::prelude::*;
use dioxus_material_icons::MaterialIconStylesheet;

use crate::MAIN_CSS;

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
    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "CREATE ACCOUNT SESSION" }
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
            div { style: "display: inline-block; margin-bottom: 36px;" }
            div {
                id: "column-section",
                button {
                    id: "button",
                    "Create",
                    // value: "{input_text}",
                    // oninput: move |event| input_text.set(event.value())
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
