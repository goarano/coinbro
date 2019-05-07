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

fn crypto_request<T: IntoUrl>(url: T) -> Result<CryptowatchResponse, Error> {
    let mut response = reqwest::get(url)?;
    let res_json: CryptowatchResponse = response.json().unwrap();
    println!("{:?}", res_json);
    Ok(res_json)
}

pub fn summaries() -> Result<Value, Error> {
    let url_str = "https://api.cryptowat.ch/markets/summaries";
    let response = crypto_request(url_str)?;

    Ok(response.result)
}
