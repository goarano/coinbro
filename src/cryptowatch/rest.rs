use reqwest::{Error, IntoUrl};
use crate::cryptowatch::data::*;
use serde_json::Value;

fn crypto_request<T: IntoUrl>(url: T) -> Result<CryptowatchResponse, Error> {
    let mut response = reqwest::get(url)?;
    let res_json: CryptowatchResponse = response.json().unwrap();
    println!("{:?}", res_json);
    Ok(res_json)
}

pub fn summaries() -> Result<Value, Error> {
    let url_str = "https://api.cryptowat.ch/markets/summaries";
    let response = crypto_request(url_str)?;

    let result = response.result.as_object().unwrap();
    let x: MarketSummary =
        serde_json::from_value(result.get("binance:ambeth").unwrap().clone()).unwrap();
    println!("{:?}", x);

    Ok(response.result)
}
