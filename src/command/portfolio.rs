use crate::cryptowatch::client::Cryptowatch;
use crate::errors::{Error, ErrorKind, Result};
use crate::output::output_summary_table;
use dirs::config_dir;
use itertools::Itertools;
use std::path::{Path, PathBuf};

pub const PORTFOLIO_CONFIG_FILE_NAME: &str = "coinbro.json";

pub fn run<P>(config_file: P) -> Result<()>
where
    P: Into<Option<String>>,
{
    let portfolio_config_file = config_file.into().map(PathBuf::from).unwrap_or(
        config_dir()
            .and_then(|p| {
                p.join(Path::new(PORTFOLIO_CONFIG_FILE_NAME))
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

    Ok(())
}
