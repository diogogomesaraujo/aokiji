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
            span { id: "secondary" , style: "display: inline-block; margin-bottom: 14px;", "TOTAL BALANCE" }
            div {
                style: "display: inline-block; margin-bottom: 36px;",
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
                        strong { id: "sub-heading" , "0.0005" }
                    }
                }
            }
        }
    }
}
