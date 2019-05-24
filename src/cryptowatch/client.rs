use crate::cryptowatch::data::MarketSummary;
use crate::cryptowatch::deserializer::{deserialize_market_summaries, deserialize_market_summary};
use crate::cryptowatch::errors::Error;
use crate::cryptowatch::rest::{cryptowatch_get, cryptowatch_get_multiple};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Cryptowatch {
    remaining_allowance: Rc<RefCell<usize>>,
}

impl Cryptowatch {
    pub fn new() -> Cryptowatch {
        Cryptowatch {
            remaining_allowance: Rc::new(RefCell::new(1)),
        }
    }

    fn url_builder<T>(&self, endpoint: T) -> String
    where
        T: AsRef<str>,
    {
        format!("https://api.cryptowat.ch/{}", endpoint.as_ref())
    }

    fn set_allowance(&self, remaining_allowance: usize) {
        let previous_remaining_allowance = self.remaining_allowance.replace(remaining_allowance);
        println!(
            "previous remaining allowance: {}",
            previous_remaining_allowance
        );
    }

    pub fn market_summary<T>(&self, market: T, pair: T) -> Result<MarketSummary, Error>
    where
        T: AsRef<str>,
    {
        let url_str = self.url_builder(format!(
            "markets/{}/{}/summary",
            market.as_ref(),
            pair.as_ref()
        ));
        let response = cryptowatch_get(&url_str)?;
        self.set_allowance(response.allowance.remaining);
        deserialize_market_summary(response)
    }

    pub fn market_summaries(
        &self,
    ) -> Result<HashMap<String, HashMap<String, MarketSummary>>, Error> {
        let url_str = self.url_builder("markets/summaries");
        let response = cryptowatch_get(&url_str)?;
        self.set_allowance(response.allowance.remaining);
        cryptowatch_get_multiple(&vec!["https://google.com", "https://cryptowat.ch"]);
        deserialize_market_summaries(response)
    }
}
