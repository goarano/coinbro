use crate::cryptowatch::data::*;
use itertools::Itertools;
use reqwest::{Error, IntoUrl};
use serde_json::{Map, Value};
use std::collections::HashMap;

fn crypto_request<T: IntoUrl>(url: T) -> Result<CryptowatchResponse, Error> {
    let mut response = reqwest::get(url)?;
    let res_json: CryptowatchResponse = response.json().unwrap();
    println!("{:?}", res_json);
    Ok(res_json)
}

pub fn market_summaries() -> Result<HashMap<String, HashMap<String, MarketSummary>>, Error> {
    let url_str = "https://api.cryptowat.ch/markets/summaries";
    let response = crypto_request(url_str)?;
    let mut response_map: Map<String, Value> = response.result.as_object().unwrap().to_owned();

    let keys_strings: Vec<String> = response_map.keys().cloned().collect::<Vec<_>>();
    let keys: Vec<String> = keys_strings.iter().map(ToOwned::to_owned).collect();

    let market_pairs: Vec<(String, String, MarketSummary)> = keys
        .into_iter()
        .filter_map(|k| {
            response_map
                .remove(&k)
                .and_then(|v| serde_json::from_value(v).ok())
                .map(|v: MarketSummary| (k, v))
        })
        .filter_map(|(k, v)| {
            let mut market_pair: Vec<&str> = k.split(":").collect();
            match market_pair.len() {
                2 => {
                    let market = market_pair.remove(0).to_owned();
                    let pair = market_pair.remove(0).to_owned();
                    Some((market, pair, v))
                }
                _ => {
                    panic!("This is not good");
                }
            }
        })
        .collect();

    let markets: Vec<_> = market_pairs
        .iter()
        .map(|(market, _, _)| market)
        .unique()
        .collect();
    println!("{:?}", markets);

    let pairs: Vec<_> = market_pairs
        .iter()
        .map(|(_, pair, _)| pair)
        .unique()
        .collect();
    println!("{:?}", pairs);

    let mut market_pair_map: HashMap<String, HashMap<String, MarketSummary>> = HashMap::new();
    market_pairs
        .into_iter()
        .for_each(|(market, pair, summary)| {
            market_pair_map
                .entry(market)
                .or_insert(HashMap::new())
                .insert(pair, summary);
        });

    Ok(market_pair_map)
}
