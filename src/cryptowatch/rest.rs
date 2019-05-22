use crate::cryptowatch::data::*;
use futures::{stream, Future, Stream};
//use hyper::{Client, Uri};
use reqwest::r#async::{Chunk, Client, Response};
use reqwest::{Error, IntoUrl};
use std::sync::{Arc, Mutex};
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
                .and_then(|r: Response| r.into_body().concat2().from_err())
        })
        .buffer_unordered(4);
    let res_arc = Arc::new(Mutex::new(Vec::<Result<String, String>>::new()));
    let work = bodies
        .for_each({
            let res = Arc::clone(&res_arc);
            move |p: Chunk| {
                println!("work done: {:?}", &p);
                let v = p.to_vec();
                let b = String::from_utf8_lossy(&v).to_string();
                res.lock().unwrap().push(Ok(b));
                Ok(())
            }
        })
        .map_err({
            let res = Arc::clone(&res_arc);
            move |e| {
                println!("work not done: {}", e);
                res.lock().unwrap().push(Err(e.to_string()));
            }
        });

    tokio::run(work);

    let res = res_arc.lock().unwrap();
    println!("{:?}", &res_arc);
}
