use crate::cryptowatch::data::*;
use futures::{stream, Future, Stream};
//use hyper::{Client, Uri};
use reqwest::r#async::Client;
use reqwest::{Error, IntoUrl};
use tokio;

pub fn cryptowatch_get<T>(url: T) -> Result<CryptowatchResponse, Error>
where
    T: IntoUrl + AsRef<str>,
{
    let mut response = reqwest::get(url)?;
    let res_json: CryptowatchResponse = response.json().unwrap();
    Ok(res_json)
}

/*
pub fn cryptowatch_get_future() {
    let client = Client::new();
    let urls = vec!["url1", "url2"];
    let bodies = stream::iter_ok(urls)
        .map(move |url| {
            client
                .get(Uri::from_static(url))
                .and_then(|r| r.into_body().concat2())
        })
        .buffer_unordered(4);
    let work = bodies.for_each(|p| println!("{:?}", p));
    tokio::run(work);
}
*/

pub fn cryptowatch_get_future() {
    let client = Client::new();
    let urls = vec!["https://www.cryptowat.ch", "https://goasdfasdfogle.com"];
    let bodies = stream::iter_ok(urls)
        .map(move |url| {
            client
                .get(url)
                .send()
                .and_then(|r| r.into_body().concat2().from_err())
        })
        .buffer_unordered(4);
    let work = bodies
        .for_each(|p| {
            println!("work done: {:?}", &p);
            Ok(())
        })
        .map_err(|e| println!("work not done: {}", e));
    tokio::run(work);
}
