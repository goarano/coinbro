use crate::cryptowatch::data::*;
use crate::cryptowatch::errors::{Error, ErrorKind};
use itertools::Itertools;
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

    println!("{:?}", markets.iter().sorted().collect::<Vec<_>>());
    println!("{:?}", pairs.iter().sorted().collect::<Vec<_>>());

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_market_summaries() {
        let summary: Value = serde_json::from_str(
            "{
                \"result\": {
                    \"binance:adabnb\": {
                        \"price\": {
                            \"last\": 0.00318,
                            \"high\": 0.00357,
                            \"low\": 0.00305,
                            \"change\": {
                                \"percentage\": -0.05357143,
                                \"absolute\": -0.00018
                            }
                        },
                        \"volume\": 18708919.5,
                        \"volumeQuote\": 61375.854008
                    },
                    \"binance:adabtc\": {
                        \"price\": {
                            \"last\": 0.00001107,
                            \"high\": 0.00001178,
                            \"low\": 0.00001051,
                            \"change\": {
                                \"percentage\": 0.009107468,
                                \"absolute\": 1e-7
                            }
                        },
                        \"volume\": 315125414,
                        \"volumeQuote\": 3491.26739894
                    }
                },
                \"allowance\": {
                    \"cost\": 81720081,
                    \"remaining\": 7726941362
                }
            }",
        )
        .unwrap();
        let response = CryptowatchResponse {
            result: summary,
            allowance: Allowance {
                cost: 0,
                remaining: 0,
            },
        };

        let summaries_res = deserialize_market_summaries(response);
        assert!(summaries_res.is_ok());

        let summaries = summaries_res.unwrap();

        assert_eq!(summaries.keys().len(), 1);
        assert!(summaries.contains_key("binance"));

        let binance = summaries.get("binance").unwrap();
        assert_eq!(binance.keys().len(), 2);
        assert!(binance.contains_key("adabtc"));
    }
}
