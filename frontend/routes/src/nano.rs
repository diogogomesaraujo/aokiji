//! This file contains all the structures and api calls to the Nano RPC API

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

/// Define the struct for the get_account_balance API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct AccountBalanceResponse {
    balance: String,
    pending: String,
    receivable: String,
    balance_nano: String,
    pending_nano: String,
    receivable_nano: String,
}

/// Define the struct for the content of the block in the get_block_info API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct BlockInfoContent {
    #[serde(rename = "type")]
    content_type: String,
    account: String,
    previous: String,
    representative: String,
    balance: String,
    balance_nano: String,
    link: String,
    link_as_account: String,
    signature: String,
    work: String,
}

/// Define the struct for the get_block_info API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct BlockInfoResponse {
    block_account: String,
    amount: String,
    amount_nano: String,
    balance: String,
    balance_nano: String,
    height: String,
    local_timestamp: String,
    successor: String,
    confirmed: String,
    content: BlockInfoContent,
    subtype: String,
}

/// Define the struct for the wallet_create API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct WalletCreateResponse {
    wallet: String,
}

/// Define the struct for the wallet_destroy API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct WalletDestroyResponse {
    destroy: String,
}

/// Define the struct for the account_create API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct AccountCreateResponse {
    account: String,
}

/// Define the struct for the account_destroy API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct AccountDestroyResponse {
    remove: String,
}

/// Function that gets the version information from the Nano API.
pub async fn get_version() -> VersionResponse {
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [("action", "version")].into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    response.json::<VersionResponse>().await.unwrap()
}

/// Function that gets an account's information from the Nano API.
pub async fn get_account_info(account: &str) -> AccountInfoResponse {
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [("action", "account_info"), ("account", account)].into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    response.json::<AccountInfoResponse>().await.unwrap()
}

/// Function that gets an account's history (according to a block count) from the Nano API.
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

/// Function that gets an account's balance from the Nano API.
pub async fn get_account_balance(account: &str) -> AccountBalanceResponse {
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [("action", "account_info"), ("account", account)].into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    response.json::<AccountBalanceResponse>().await.unwrap()
}

/// Function that gets a block's info from the Nano API.
pub async fn get_block_info(hash: &str) -> BlockInfoResponse {
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [("action", "block_info"), ("hash", hash)].into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    response.json::<BlockInfoResponse>().await.unwrap()
}

/// Function that creates a wallet in for Nano blockchain.
pub async fn wallet_create() -> WalletCreateResponse {
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [("action", "wallet_create")].into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    response.json::<WalletCreateResponse>().await.unwrap()
}

/// Function that destroys a wallet in for Nano blockchain.
pub async fn wallet_destroy() -> WalletDestroyResponse {
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [("action", "wallet_destroy")].into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    response.json::<WalletDestroyResponse>().await.unwrap()
}

/// Function that creates an account in a given wallet.
pub async fn account_create(wallet: &str) -> AccountCreateResponse {
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [("action", "account_create"), ("wallet", wallet)].into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    response.json::<AccountCreateResponse>().await.unwrap()
}

/// Function that removes an account in a given wallet.
pub async fn account_remove(wallet: &str, account: &str) -> AccountDestroyResponse {
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [
        ("action", "account_create"),
        ("wallet", wallet),
        ("account", account),
    ]
    .into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    response.json::<AccountDestroyResponse>().await.unwrap()
}
