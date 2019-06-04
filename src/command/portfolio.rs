use crate::command::portfolio_config::PortfolioConfig;
use crate::cryptowatch::client::Cryptowatch;
use crate::cryptowatch::data::MarketSummary;
use crate::errors::{Error, ErrorKind, Result};
use crate::output::output_summary_table;
use dirs::config_dir;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub const PORTFOLIO_CONFIG_FILE_DIR: &str = "coinbro";
pub const PORTFOLIO_CONFIG_FILE_NAME: &str = "config.json";

pub fn run<P>(config_file: P) -> Result<()>
where
    P: Into<Option<String>>,
{
    let portfolio_config_file = config_file.into().map(PathBuf::from).unwrap_or(
        config_dir()
            .and_then(|p| {
                p.join(Path::new(PORTFOLIO_CONFIG_FILE_DIR))
                    .join(Path::new(PORTFOLIO_CONFIG_FILE_NAME))
                    .into_os_string()
                    .into_string()
                    .ok()
            })
            .map(PathBuf::from)
            .expect("No config dir found for this OS"),
    );
    if !portfolio_config_file.exists() || !portfolio_config_file.is_file() {
        bail!(ErrorKind::ConfigFileNotFound(
            portfolio_config_file
                .into_os_string()
                .into_string()
                .unwrap_or(String::from("cannot read config file path"))
        ));
    }

    let portfolio_config = read_portfolio_config(portfolio_config_file)?;

    let client = Cryptowatch::new();
    let summaries = client.all_market_summaries()?;
    let exchange = "kraken";
    let kraken = summaries
        .get(exchange)
        .ok_or::<Error>(ErrorKind::ExchangeNotFound(String::from(exchange)).into())?;

    let pairs = portfolio_config
        .portfolios
        .iter()
        .map(|e| e.crypto.to_string())
        .map(|e| e.to_lowercase() + &portfolio_config.base_currency.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let summaries: Vec<&MarketSummary> = pairs
        .iter()
        .map(|pair| {
            kraken
                .get(pair)
                .ok_or::<Error>(ErrorKind::PairNotFound(pair.clone()).into())
        })
        .collect::<Result<_>>()?;

    output_summary_table(summaries.as_slice());

    Ok(())
}

fn read_portfolio_config<F>(portfolio_config_file: F) -> Result<PortfolioConfig>
where
    F: AsRef<Path>,
{
    let mut file = File::open(portfolio_config_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    serde_json::from_str(&contents).map_err(Into::into)
}
