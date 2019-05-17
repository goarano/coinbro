use crate::cryptowatch::data::*;
use crate::cryptowatch::errors::{Error, ErrorKind};
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};

pub fn deserialize_market_summaries(
    response: CryptowatchResponse,
) -> Result<HashMap<String, HashMap<String, MarketSummary>>, Error> {
    let mut response_map: Map<String, Value> = response
        .result
        .as_object()
        .ok_or(ErrorKind::ParseError(String::from("something went wrong")))?
        .to_owned();

    let keys_strings: Vec<String> = response_map.keys().cloned().collect();
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

    let (markets, pairs): (HashSet<_>, HashSet<_>) = market_pairs
        .iter()
        .map(|(m, p, _)| (m.clone(), p.clone()))
        .unzip();

    println!("{:?}", markets);
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
