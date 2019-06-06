use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Allowance {
    pub cost: usize,
    pub remaining: usize,
}

#[derive(Deserialize, Debug)]
pub struct CryptowatchResponse {
    pub result: Value,
    pub allowance: Allowance,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MarketPriceChange {
    pub percentage: f32,
    pub absolute: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MarketPrice {
    pub last: f32,
    pub high: f32,
    pub low: f32,
    pub change: MarketPriceChange,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarketSummary {
    pub price: MarketPrice,
    pub volume: f32,
    pub volume_quote: Option<f32>,
}

#[derive(EnumString, EnumIter, Display, Debug)]
pub enum Market {
    #[strum(serialize = "binance")]
    Binance,
    #[strum(serialize = "bitfinex")]
    Bitfinex,
    #[strum(serialize = "kraken")]
    Kraken,
}
