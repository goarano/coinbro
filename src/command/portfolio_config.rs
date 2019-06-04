use serde::Deserialize;

#[derive(EnumString, EnumIter, Display, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Fiat {
    CHF,
    USD,
    EUR,
}

#[derive(EnumString, EnumIter, Display, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Crypto {
    BTC,
    ETH,
    ZEC,
    XMR,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioEntry {
    pub crypto: Crypto,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioConfig {
    /// desired currency by the user
    #[serde(default = "PortfolioConfig::default_quote_currency")]
    pub quote_currency: Fiat,
    /// default exchange currency
    #[serde(default = "PortfolioConfig::default_base_currency")]
    pub base_currency: Fiat,
    #[serde(default = "PortfolioConfig::default_exchange")]
    pub exchange: String,
    pub portfolios: Vec<PortfolioEntry>,
}

impl PortfolioConfig {
    pub fn default_quote_currency() -> Fiat {
        Fiat::CHF
    }

    pub fn default_base_currency() -> Fiat {
        Fiat::EUR
    }

    pub fn default_exchange() -> String {
        "kraken".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_portfolio_serialization() {
        let test_json = json!({
            "quoteCurrency": "CHF",
            "baseCurrency": "EUR",
            "exchange": "kraken",
            "portfolios": []
        });

        let config = PortfolioConfig {
            quote_currency: Fiat::CHF,
            base_currency: Fiat::EUR,
            exchange: "kraken".to_string(),
            portfolios: vec![],
        };

        let config_val = serde_json::to_value(&config).unwrap();

        assert_json_eq!(test_json, config_val);
    }

    #[test]
    fn test_portfolio_config_default_values() {
        let test_json = json!({
            "portfolios": []
        });

        let config: PortfolioConfig =
            serde_json::from_value(test_json).expect("serde_json deserialize error");

        assert_eq!(
            config.quote_currency,
            PortfolioConfig::default_quote_currency()
        );
        assert_eq!(
            config.base_currency,
            PortfolioConfig::default_base_currency()
        );
        assert_eq!(config.exchange, PortfolioConfig::default_exchange());
    }
}
