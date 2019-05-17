use crate::cryptowatch::data::MarketSummary;
use prettytable::Table;

pub fn output_summary_table(summaries: &[&MarketSummary]) {
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
        let price = &summary.price;
        let volume = summary.volume;
        let volume_quote = summary
            .volume_quote
            .map_or(String::new(), |q| q.to_string());
        table.add_row(row![
            from,
            to,
            price.last,
            price.high,
            price.low,
            price.change.absolute,
            price.change.percentage,
            volume,
            volume_quote
        ]);
    });

    table.printstd();
}
