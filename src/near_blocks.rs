use std::error;

use reqwasm::http::Request;
use serde::Deserialize;

const PRICE_URL: &str = "https://nearblocks.io/api/near-price";
const TXS_URL: &str = "https://nearblocks.io/api/txns?limit=25&offset=0";

#[derive(Default, Deserialize)]
pub struct Price {
    pub usd: f64,
    pub btc: f64,
}

#[derive(Default, Deserialize)]
pub struct Transaction {
    pub transaction_hash: String,
    #[serde(rename = "type")]
    pub tx_type: String,
    pub height: u64,
    pub included_in_block_hash: String,
    pub block_timestamp: u64,
    pub from: String,
    pub to: String,
    pub deposit_value: String,
    pub transaction_fee: String,
}

#[derive(Default, Deserialize)]
pub struct Transactions {
    pub txns: Vec<Transaction>,
}

pub async fn get_price() -> Result<Price, Box<dyn error::Error>> {
    let resp = Request::get(PRICE_URL).send().await?;
    match resp.status() {
        200 => Ok(resp.json::<Price>().await?),
        code => Err(format!(
            "{}: Invalid REST API response code {}",
            PRICE_URL, code
        ))?,
    }
}

pub async fn get_transactions() -> Result<Transactions, Box<dyn error::Error>> {
    let resp = Request::get(TXS_URL).send().await?;
    match resp.status() {
        200 => Ok(resp.json::<Transactions>().await?),
        code => Err(format!(
            "{}: Invalid REST API response code {}",
            TXS_URL, code
        ))?,
    }
}
