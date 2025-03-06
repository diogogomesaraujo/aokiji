//! This file contains all the structures and api calls to the Nano RPC API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Define the current Nano RPC API link as constant (The link can be changed since it follows Nano RPC guidelines).
const URL: &str = "https://rpc.nano.to";

/// Define the struct for the get_version API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct VersionResponse {
    rpc_version: Option<String>,
    store_version: Option<String>,
    protocol_version: Option<String>,
    node_vendor: Option<String>,
    store_vendor: Option<String>,
    network: Option<String>,
}

/// Define the struct for the get_account_info API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct AccountInfoResponse {
    frontier: Option<String>,
    open_block: Option<String>,
    representative_block: Option<String>,
    balance: Option<String>,
    balance_nano: Option<String>,
    modified_timestamp: Option<String>,
    block_count: Option<String>,
    account_version: Option<String>,
    confirmation_height: Option<String>,
    confirmation_height_frontier: Option<String>,
}

/// Define the struct for each node of the history in the get_account_history API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct AccountHistoryNode {
    #[serde(rename = "type")]
    history_type: Option<String>,
    account: Option<String>,
    amount: Option<String>,
    local_timestamp: Option<String>,
    height: Option<String>,
    hash: Option<String>,
    confirmed: Option<String>,
    #[serde(default)]
    username: Option<String>,
}

/// Define the struct for the get_account_history API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct AccountHistoryResponse {
    account: Option<String>,
    history: Option<Vec<AccountHistoryNode>>,
    #[serde(default)]
    previous: Option<String>,
}

/// Define the struct for the get_account_balance API call response.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AccountBalanceResponse {
    pub balance: Option<String>,
    pub pending: Option<String>,
    pub receivable: Option<String>,
    pub balance_nano: Option<String>,
    pub pending_nano: Option<String>,
    pub receivable_nano: Option<String>,
}

/// Implement functions for AccountBalanceResponse.
impl AccountBalanceResponse {
    /// Implement new to create empty AccountBalanceResponses.
    pub fn new() -> Self {
        Self {
            balance: Some(String::new()),
            pending: Some(String::new()),
            receivable: Some(String::new()),
            balance_nano: Some(String::new()),
            pending_nano: Some(String::new()),
            receivable_nano: Some(String::new()),
        }
    }
}

/// Define the struct for the account_create API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct AccountCreateResponse {
    account: Option<String>,
}

/// Define the struct for the account_destroy API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct AccountDestroyResponse {
    remove: Option<String>,
}

/// Define the struct for the get_wallet_info API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct WalletInfoResponse {
    balance: Option<String>,
    pending: Option<String>,
    recievable: Option<String>,
    accounts_count: Option<String>,
    adhoc_count: Option<String>,
    deterministic_count: Option<String>,
    deterministic_index: Option<String>,
    accounts_block_count: Option<String>,
    accounts_cemented_block_count: Option<String>,
}

/// Define the struct for each node of the history in the get_account_history API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct WalletHistoryNode {
    #[serde(rename = "type")]
    history_type: Option<String>,
    account: Option<String>,
    amount: Option<String>,
    local_timestamp: Option<String>,
    hash: Option<String>,
}

/// Define the struct for the get_account_history API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct WalletHistoryResponse {
    history: Option<Vec<AccountHistoryNode>>,
}

/// Define the struct for the wallet_create API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct WalletCreateResponse {
    wallet: Option<String>,
}

/// Define the struct for the wallet_destroy API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct WalletDestroyResponse {
    destroy: Option<String>,
}

/// Define the struct for the content of the block in the get_block_info API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct BlockInfoContent {
    #[serde(rename = "type")]
    content_type: Option<String>,
    account: Option<String>,
    previous: Option<String>,
    representative: Option<String>,
    balance: Option<String>,
    balance_nano: Option<String>,
    link: Option<String>,
    link_as_account: Option<String>,
    signature: Option<String>,
    work: Option<String>,
}

/// Define the struct for the get_block_info API call response.
#[derive(Deserialize, Serialize, Debug)]
pub struct BlockInfoResponse {
    block_account: Option<String>,
    amount: Option<String>,
    amount_nano: Option<String>,
    balance: Option<String>,
    balance_nano: Option<String>,
    height: Option<String>,
    local_timestamp: Option<String>,
    successor: Option<String>,
    confirmed: Option<String>,
    content: Option<BlockInfoContent>,
    subtype: Option<String>,
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

/// Function that gets a wallet's information from the Nano API.
pub async fn get_wallet_info(wallet: &str) -> WalletInfoResponse {
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [("action", "wallet_info"), ("wallet", wallet)].into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    response.json::<WalletInfoResponse>().await.unwrap()
}

/// Function that gets a wallets's history (with all it's accounts) from the Nano API.
pub async fn get_wallet_history(wallet: &str) -> WalletHistoryResponse {
    let client = reqwest::Client::new();
    let data: HashMap<_, _> = [("action", "wallet_history"), ("wallet", wallet)].into();
    let response = client.post(URL).json(&data).send().await.unwrap();

    response.json::<WalletHistoryResponse>().await.unwrap()
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
