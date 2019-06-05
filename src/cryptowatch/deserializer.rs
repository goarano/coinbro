use crate::cryptowatch::data::*;
use crate::cryptowatch::errors::{ErrorKind, Result};
use itertools::Itertools;
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};

pub fn deserialize_all_market_summaries(
    response: CryptowatchResponse,
) -> Result<HashMap<String, HashMap<String, MarketSummary>>> {
    let mut response_map: Map<String, Value> = response
        .result
        .as_object()
        .ok_or(ErrorKind::ParseError(String::from("something went wrong")))?
        .to_owned();

    let market_pairs_res: Result<Vec<(String, String, MarketSummary)>> = response_map
        .keys()
        .cloned()
        .collect_vec()
        .into_iter()
        .map(|key| {
            response_map
                .remove(&key)
                .map(|value| {
                    let summary: MarketSummary = serde_json::from_value(value)?;
                    Ok((key, summary))
                })
                .unwrap()
        })
        .map(|result: Result<(String, MarketSummary)>| {
            let (key, summary) = result?;
            let mut market_pair: Vec<&str> = key.split(":").collect();
            match market_pair.len() {
                2 => {
                    let market = market_pair.remove(0).to_owned();
                    let pair = market_pair.remove(0).to_owned();
                    Ok((market, pair, summary))
                }
                _ => Err(ErrorKind::ParseError(format!(
                    "cannot parse market pair {:?}",
                    market_pair
                ))
                .into()),
            }
        })
        .collect();
    let market_pairs = market_pairs_res?;

    let (markets, pairs): (HashSet<_>, HashSet<_>) = market_pairs
        .iter()
        .map(|(m, p, _)| (m.clone(), p.clone()))
        .unzip();

    debug!("markets: {:?}", markets.iter().sorted().collect::<Vec<_>>());
    debug!("pairs: {:?}", pairs.iter().sorted().collect::<Vec<_>>());

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

pub fn deserialize_market_summary(response: CryptowatchResponse) -> Result<MarketSummary> {
    serde_json::from_value(response.result).map_err(|e| e.into())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_market_summaries() {
        let response: CryptowatchResponse = serde_json::from_str(
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

        let summaries_res = deserialize_all_market_summaries(response);
        assert!(summaries_res.is_ok(), format!("{:?}", summaries_res));

        let summaries = summaries_res.unwrap();

        assert_eq!(summaries.keys().len(), 1);
        assert!(summaries.contains_key("binance"));

        let binance = summaries.get("binance").unwrap();
        assert_eq!(binance.keys().len(), 2);
        assert!(binance.contains_key("adabtc"));
    }

    #[test]
    fn test_deserialize_market_summary() {
        let response: CryptowatchResponse = serde_json::from_str(
            "{
                \"result\": {
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
                \"allowance\": {
                    \"cost\": 81720081,
                    \"remaining\": 7726941362
                }
            }",
        )
        .unwrap();

        let summaries_res = deserialize_market_summary(response);
        assert!(summaries_res.is_ok(), format!("{:?}", summaries_res));

        let summary = summaries_res.unwrap();
        assert_eq!(summary.volume, 18708919.5);
    }
}
