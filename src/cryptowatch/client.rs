use crate::cryptowatch::data::MarketSummary;
use crate::cryptowatch::deserializer::market_summaries;
use crate::cryptowatch::errors::Error;
use crate::cryptowatch::rest::cryptowatch_get;
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

    pub fn market_summaries(
        &self,
    ) -> Result<HashMap<String, HashMap<String, MarketSummary>>, Error> {
        let url_str = self.url_builder("markets/summaries");
        let response = cryptowatch_get(&url_str)?;
        let previous_remaining_allowance = self
            .remaining_allowance
            .replace(response.allowance.remaining);
        println!(
            "previous remaining allowance: {}",
            previous_remaining_allowance
        );
        market_summaries(response)
    }
}
