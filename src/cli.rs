use clap::{App, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("coinbro")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Your one-in-all CLI tool for everything crypto -> written in Rust ❤")
        .subcommand(SubCommand::with_name("stats").about("Latest global crypto stats"))
}
