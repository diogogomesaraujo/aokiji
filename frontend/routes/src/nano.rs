use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Define the current Nano RPC API link as constant (The link can be changed since it follows Nano RPC guidelines).
const URL: &str = "https://rpc.nano.to";

/// Define the struct for the get_version API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct VersionResponse {
    rpc_version: String,
    store_version: String,
    protocol_version: String,
    node_vendor: String,
    store_vendor: String,
    network: String,
}

/// Define the struct for the get_account_info API call response.
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

/// Define the struct for each node of the history in the get_account_history API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct HistoryNode {
    #[serde(rename = "type")]
    history_type: String,
    account: String,
    amount: String,
    local_timestamp: String,
    height: String,
    hash: String,
    confirmed: String,
    #[serde(default)]
    username: Option<String>,
}

/// Define the struct for the get_account_history API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct AccountHistoryResponse {
    account: String,
    history: Vec<HistoryNode>,
    #[serde(default)]
    previous: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccountBalanceResponse {
    balance: String,
    pending: String,
    receivable: String,
    balance_nano: String,
    pending_nano: String,
    receivable_nano: String,
}

// CALLS TO NANO RPC API

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

pub async fn get_account_history(account: &str, count: i32) -> AccountHistoryResponse {
    let count = format!("{}", count);
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [
        ("action", "account_history"),
        ("account", account),
        ("count", count.as_str()),
    ]
    .into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    response.json::<AccountHistoryResponse>().await.unwrap()
}

pub async fn get_account_balance(account: &str) -> AccountBalanceResponse {
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [("action", "account_info"), ("account", account)].into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    response.json::<AccountBalanceResponse>().await.unwrap()
}
