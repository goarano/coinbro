use crate::cryptowatch::data::*;
use futures::{stream, Future, Stream};
//use hyper::{Client, Uri};
use crate::cryptowatch::errors::Result;
use reqwest::r#async::{Chunk, Client, Response};
use reqwest::IntoUrl;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use tokio;
use tokio::runtime::current_thread::Builder;

pub fn cryptowatch_get<T>(url: T) -> Result<CryptowatchResponse>
where
    T: IntoUrl + AsRef<str>,
{
    let mut response = reqwest::get(url)?;
    let res_json: CryptowatchResponse = response.json().unwrap();
    Ok(res_json)
}

pub fn cryptowatch_get_multiple<T>(urls: &[T]) -> HashMap<T, Result<CryptowatchResponse>>
where
    T: AsRef<str> + Clone + Eq + Hash,
{
    let client = Client::new();
    //let t = stream::iter_ok(urls);
    let bodies = stream::iter_ok(urls)
        .map(|url| {
            client
                .get(url.as_ref())
                .send()
                .and_then(|r: Response| r.into_body().concat2().from_err())
        })
        .buffer_unordered(4);
    let res_arc = Arc::new(Mutex::new(Vec::<Result<String>>::new()));
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

    let mut runtime = Builder::new().build().unwrap();
    let _run_res = runtime.block_on(work);

    let res = Arc::try_unwrap(res_arc).unwrap().into_inner().unwrap(); // This cannot possibly fail
    debug!("res: {:?}", &res);

    urls.iter()
        .cloned()
        .zip(res.into_iter().map(|r| {
            r.and_then(|s| {
                println!("toparse: {:?}", &s);
                //TODO error handling eg. for
                // "{\"error\":\"Exchange not found\",\"allowance\":{\"cost\":3167051,\"remaining\":6883695512}}\n"
                serde_json::from_str::<CryptowatchResponse>(&s).map_err(Into::into)
            })
        }))
        .collect::<HashMap<T, Result<CryptowatchResponse>>>()
}
