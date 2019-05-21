use clap::{App, Arg, SubCommand};

pub const STATS_ARG: &str = "stats";
pub const PORTFOLIO_ARG: &str = "portfolio";
pub const PORTFOLIO_CONFIG_ARG: &str = "config";

pub fn build_cli() -> App<'static, 'static> {
    App::new("coinbro")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Your one-in-all CLI tool for everything crypto -> written in Rust ❤")
        .subcommand(SubCommand::with_name(STATS_ARG).about("Latest global crypto stats"))
        .subcommand(
            SubCommand::with_name(PORTFOLIO_ARG)
                .about("Porfolio summary")
                .arg(Arg::with_name(PORTFOLIO_CONFIG_ARG)),
        )
}
