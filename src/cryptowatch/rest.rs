use crate::cryptowatch::data::*;
use futures::{stream, Future, Stream};
//use hyper::{Client, Uri};
use crate::cryptowatch::errors::Error;
use reqwest::r#async::{Chunk, Client, Response};
use reqwest::IntoUrl;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use tokio;
use tokio::runtime::current_thread::Builder;

pub fn cryptowatch_get<T>(url: T) -> Result<CryptowatchResponse, Error>
where
    T: IntoUrl + AsRef<str>,
{
    let mut response = reqwest::get(url)?;
    let res_json: CryptowatchResponse = response.json().unwrap();
    Ok(res_json)
}

pub fn cryptowatch_get_multiple<T>(urls: &[T]) -> HashMap<T, Result<CryptowatchResponse, Error>>
where
    T: IntoUrl + AsRef<str> + Clone + Eq + Hash,
{
    let client = Client::new();
    //let t = stream::iter_ok(urls);
    let bodies = stream::iter_ok(urls)
        .map(|url| {
            client
                .get(url.clone())
                .send()
                .and_then(|r: Response| r.into_body().concat2().from_err())
        })
        .buffer_unordered(4);
    let res_arc = Arc::new(Mutex::new(Vec::<Result<String, Error>>::new()));
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
                res.lock().unwrap().push(Err(e.into()));
            }
        });

    //let mut reactor = Core::new().unwrap();
    //tokio::run(work);
    let mut runtime = Builder::new().build().unwrap();
    let run_res = runtime.block_on(work);

    let res = res_arc.lock().unwrap();
    debug!("res: {:?}", &res);

    let res2 = res.iter().map(|r: &Result<String, Error>| match r {
        Ok(s) => serde_json::from_str::<CryptowatchResponse>(&s).map_err(|e| Error::from(e)),
        Err(e) => Err(Error::from(e.to_string())), //TODO
    });
    let res3 = urls
        .iter()
        .cloned()
        .zip(res2)
        .collect::<HashMap<T, Result<CryptowatchResponse, _>>>();

    res3
}
