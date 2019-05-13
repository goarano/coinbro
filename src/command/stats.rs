use crate::cryptowatch::rest::summaries;
use prettytable::{Cell, Row, Table};

pub fn run() {
    let summaries = summaries().unwrap();

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
}