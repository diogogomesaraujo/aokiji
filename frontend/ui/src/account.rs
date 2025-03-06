use dioxus::prelude::*;

const ACCOUNT_CSS: Asset = asset!("assets/styling/account.css");

pub fn Account() -> Element {
    rsx! {
        Balance{}
    }
}

pub fn Balance() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: ACCOUNT_CSS }
        div {
            id: "card",
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 20px;", "TOTAL BALANCE" }
            div {
                id: "container",
                div {
                    id: "fill-card",
                    span { id: "sub-heading" , "XNO" }
                    strong { id: "h1" , "0.0005" }
                }
                div {
                    id: "fill-card",
                    span { id: "secondary" , "~EUR" }
                    div {
                        id: "secondary" ,
                        strong { id: "sub-heading" , "0.0005€" }
                    }
                }
            }
        }
    }
}
