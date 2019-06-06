use serde::Deserialize;
use serde_json::Value;

#[derive(Display, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum EitherFiatOrCrypto {
    Fiat(Fiat),
    Crypto(Crypto),
}

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

#[derive(Deserialize, Debug)]
pub struct FromToPair {
    pub from: EitherFiatOrCrypto,
    pub to: EitherFiatOrCrypto,
}
