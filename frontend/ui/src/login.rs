use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("assets/styling/main.css");

#[component]
pub fn Login() -> Element {
    rsx! {
    document::Link { rel: "stylesheet", href: MAIN_CSS }
    LoginForm{}
    }
}

#[component]
fn LoginForm() -> Element {
    rsx! {
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 36px;", "LOGIN TO ACCOUNT" }
            div {
                id: "column-section",
                span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Email:" }
                input {
                    id: "input",
                    r#type: "email",
                    // value: "{input_text}",
                    // oninput: move |event| input_text.set(event.value())
                }
            }
            div { style: "display: inline-block; margin-bottom: 14px;" }
            div {
                id: "column-section",
                span { id: "sub-heading", style: "display: inline-block; margin-bottom: 8px;", "Password:" }
                input {
                    id: "input",
                    r#type: "password",
                    // value: "{input_text}",
                    // oninput: move |event| input_text.set(event.value())
                }
            }
            div { style: "display: inline-block; margin-bottom: 36px;" }
            div {
                id: "column-section",
                button {
                    id: "button",
                    strong { "LOGIN" },
                    // value: "{input_text}",
                    // oninput: move |event| input_text.set(event.value())
                }
            }
        }
    }
}
