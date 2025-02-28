use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const URL: &str = "https://rpc.nano.to";

#[derive(Deserialize, Serialize, Debug)]
pub struct VersionResponse {
    rpc_version: String,
    store_version: String,
    protocol_version: String,
    node_vendor: String,
    store_vendor: String,
    network: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccountInfoResponse {
    frontier: String,
    open_block: String,
    representative_block: String,
    balance: String,
    balance_nano: String,
    modified_timestamp: String,
    block_count: String,
    account_version: String,
    confirmation_height: String,
    confirmation_height_frontier: String,
}

pub async fn get_version() -> VersionResponse {
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [("action", "version")].into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    response.json::<VersionResponse>().await.unwrap()
}

pub async fn get_account_info(account: &str) -> AccountInfoResponse {
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [("action", "account_info"), ("account", account)].into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    response.json::<AccountInfoResponse>().await.unwrap()
}

pub async fn get_account_history(
    account: &str,
    count: &i32,
) -> Result<reqwest::Response, reqwest::Error> {
    let count = format!("{}", count);
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [
        ("action", "account_history"),
        ("account", account),
        ("count", count.as_str()),
    ]
    .into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    Ok(response)
}
