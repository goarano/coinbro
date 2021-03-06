use crate::cryptowatch::client::Cryptowatch;
use crate::errors::{Error, ErrorKind, Result};
use crate::output::legacy_output_summary_table;
use itertools::Itertools;

pub fn run() -> Result<()> {
    let client = Cryptowatch::new();
    let summaries = client.all_market_summaries()?;
    let exchange = "kraken";
    let kraken = summaries
        .get(exchange)
        .ok_or::<Error>(ErrorKind::ExchangeNotFound(String::from(exchange)).into())?;
    println!("{:?}", kraken.keys().sorted().collect::<Vec<_>>());
    let pair = "ethusd";
    let summary = kraken
        .get(pair)
        .map(|s| (*s).clone())
        .ok_or::<Error>(ErrorKind::PairNotFound(String::from(pair)).into())?;

    legacy_output_summary_table(&vec![summary]);
    Ok(())
}
