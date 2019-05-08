use reqwest::{Error, IntoUrl};
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct Allowance {
    cost: usize,
    remaining: usize,
}

#[derive(Deserialize, Debug)]
struct CryptowatchResponse {
    result: Value,
    allowance: Allowance,
}

#[derive(Deserialize, Debug)]
struct MarketPriceChange {
    percentage: f32,
    absolute: f32,
}

#[derive(Deserialize, Debug)]
struct MarketPrice {
    last: f32,
    high: f32,
    low: f32,
    change: MarketPriceChange,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MarketSummary {
    price: MarketPrice,
    volume: usize,
    volume_quote: Option<f32>,
}

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
