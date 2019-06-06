use crate::cryptowatch::data::MarketSummary;
use colored::Colorize;
use prettytable::Table;

pub fn legacy_output_summary_table(summaries: &[MarketSummary]) {
    // Create the table
    let mut table = Table::new();

    table.set_titles(row![
        "from",
        "to",
        "last",
        "high",
        "low",
        "change",
        "change (%)",
        "volume",
        "volume quote"
    ]);

    summaries.iter().for_each(|summary| {
        let (from, to) = ("ETH", "USD"); //TODO this is incredibly incorrect
        add_table_entry_summary(&mut table, from, to, summary);
    });

    table.printstd();
}

fn add_table_entry_summary<S>(table: &mut Table, from: S, to: S, summary: &MarketSummary)
where
    S: AsRef<str>,
{
    let price = &summary.price;
    let volume = summary.volume;
    let volume_quote = summary
        .volume_quote
        .map_or(String::new(), |q| q.to_string());
    table.add_row(row![
        from.as_ref(),
        to.as_ref(),
        price.last,
        price.high,
        price.low,
        color_number(price.change.absolute),
        color_number(price.change.percentage),
        volume,
        volume_quote
    ]);
}

fn color_number<N>(number: N) -> String
where
    N: Into<f64> + ToString + Copy,
{
    if number.into() < 0.0 {
        number.to_string().red().to_string()
    } else if number.into() > 0.0 {
        number.to_string().green().to_string()
    } else {
        String::new()
    }
}
