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

#[cfg(test)]
mod test {
    use super::*;
    use itertools::assert_equal;
    use std::str::FromStr;
    use strum::IntoEnumIterator;

    #[test]
    fn test_strum_enum_string() {
        let btc = Crypto::from_str("BTC").unwrap();
        assert_eq!(btc, Crypto::BTC);
    }

    #[test]
    fn test_strum_enum_iter() {
        let found_btc: bool = Crypto::iter()
            .map(|c: Crypto| c.eq(&Crypto::BTC))
            .fold(false, |s: bool, e: bool| s || e);

        assert!(found_btc, "no BTC found in Enum Crypto");
    }
}
