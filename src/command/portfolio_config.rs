use serde::Deserialize;

#[derive(EnumString, EnumIter, Display, Debug, Deserialize)]
pub enum Fiat {
    CHF,
    USD,
    EUR,
}

#[derive(EnumString, EnumIter, Display, Debug, Deserialize)]
pub enum Crypto {
    BTC,
    ETH,
    ZEC,
    XMR,
}

#[derive(Deserialize, Debug)]
pub struct PortfolioEntry {
    pub crypto: Crypto,
    pub amount: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioConfig {
    /// desired currency by the user
    #[serde(default = Fiat::CHF)]
    pub quote_currency: Fiat,
    /// default exchange currency
    #[serde(default = Fiat::EUR)]
    pub base_currency: Fiat,
    pub portfolios: Vec<PortfolioEntry>,
}
