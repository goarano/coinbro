extern crate clap;
extern crate serde_json;
#[macro_use]
extern crate serde;

extern crate futures;
extern crate hyper;
extern crate tokio_core;

#[macro_use]
extern crate prettytable;

mod cli;
mod command;
mod cryptowatch;

fn main() {
    let matches = cli::build_cli().get_matches();
    match matches.subcommand() {
        ("stats", Some(argMatches)) => {
            //let summaries = cryptowatch::summaries().unwrap();
            command::stats::run();
            //println!("{}", summaries);
        }
        ("", None) => println!("No command given"),
        _ => unreachable!(),
    }
}
