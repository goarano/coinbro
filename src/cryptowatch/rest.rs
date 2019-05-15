use crate::cryptowatch::data::*;
use reqwest::{Error, IntoUrl};
use serde_json::{Map, Value};
// Needed for Market::iter()
use itertools::Itertools;
use std::collections::HashMap;
use strum::IntoEnumIterator;

fn crypto_request<T: IntoUrl>(url: T) -> Result<CryptowatchResponse, Error> {
    let mut response = reqwest::get(url)?;
    let res_json: CryptowatchResponse = response.json().unwrap();
    println!("{:?}", res_json);
    Ok(res_json)
}

pub fn market_summaries() -> Result<Value, Error> {
    let url_str = "https://api.cryptowat.ch/markets/summaries";
    let response = crypto_request(url_str)?;
    let mut response_map: Map<String, Value> = response.result.as_object().unwrap().to_owned();

    let keys_strings: Vec<String> = response_map.keys().cloned().collect::<Vec<_>>();
    let keys: Vec<&str> = keys_strings.iter().map(AsRef::as_ref).collect();

    let market_pairs: Vec<(&str, &str, MarketSummary)> = keys
        .into_iter()
        .filter_map(|k| {
            response_map
                .remove(k)
                .and_then(|v| serde_json::from_value(v).ok())
                .map(|v: MarketSummary| (k, v))
        })
        .filter_map(|(k, v)| {
            let mut market_pair: Vec<&str> = k.split(":").collect();
            match market_pair.len() {
                2 => {
                    let market = market_pair.remove(0).clone();
                    let pair = market_pair.remove(0).clone();
                    Some((market, pair, v))
                }
                _ => {
                    panic!("This is not good");
                    None
                }
            }
        })
        .collect();

    let markets: Vec<&str> = market_pairs
        .iter()
        .map(|(market, _, _)| *market)
        .unique()
        .collect();
    println!("{:?}", markets);

    let pairs: Vec<&str> = market_pairs
        .iter()
        .map(|(_, pair, _)| *pair)
        .unique()
        .collect();
    println!("{:?}", pairs);

    Ok(response.result)
}
