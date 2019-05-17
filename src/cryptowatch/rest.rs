use crate::cryptowatch::data::*;
use reqwest::{Error, IntoUrl};

pub fn cryptowatch_get<T>(url: T) -> Result<CryptowatchResponse, Error>
where
    T: IntoUrl + AsRef<str>,
{
    let mut response = reqwest::get(url)?;
    let res_json: CryptowatchResponse = response.json().unwrap();
    Ok(res_json)
}
