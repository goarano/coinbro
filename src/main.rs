extern crate clap;
extern crate serde_json;
#[macro_use]
extern crate serde;

extern crate futures;
extern crate hyper;
extern crate tokio_core;

#[macro_use]
extern crate prettytable;

extern crate strum;
#[macro_use]
extern crate strum_macros;

extern crate itertools;
#[macro_use]
extern crate error_chain;

extern crate colored;
#[macro_use]
extern crate log;
extern crate dirs;

use clap::ArgMatches;

mod cli;
mod command;
mod cryptowatch;
mod errors;
mod output;

use errors::Result;

fn main() {
    let matches = cli::build_cli().get_matches();
    match run(matches) {
        Ok(_) => println!("Bye!"),
        Err(err) => println!("error ocurred: {}", err),
    }
}

fn run(matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        (cli::STATS_ARG, Some(_arg_matches)) => {
            //let summaries = cryptowatch::summaries().unwrap();
            command::stats::run()
            //println!("{}", summaries);
        }
        (cli::PORTFOLIO_ARG, Some(arg_matches)) => {
            let config_file = arg_matches
                .value_of(cli::PORTFOLIO_CONFIG_ARG)
                .map(ToOwned::to_owned);
            command::portfolio::run(config_file)
        }
        ("", None) => bail!("No command given"),
        _ => unreachable!(),
    }
}
