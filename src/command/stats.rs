use crate::cryptowatch::client::Cryptowatch;
use crate::errors::{Error, ErrorKind, Result};
use prettytable::{Cell, Row, Table};

pub fn run() -> Result<()> {
    let client = Cryptowatch::new();
    let summaries = client.market_summaries()?;
    let kraken = summaries
        .get("kraken")
        .ok_or::<Error>(ErrorKind::Msg(String::from("kraken not found")).into())?;

    // Create the table
    let mut table = Table::new();

    // Add a row per time
    table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]);
    // A more complicated way to add a row:
    table.add_row(Row::new(vec![
        Cell::new("foobar2"),
        Cell::new("bar2"),
        Cell::new("foo2"),
    ]));

    // Print the table to stdout
    table.printstd();
    Ok(())
}
