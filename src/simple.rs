use crate::common::{build_hline, join_with_bars};

pub fn to_md_table_simple(
    headers: &csv::StringRecord,
    records: &[csv::StringRecord],
) -> Vec<String> {
    let mut table = vec![
        join_with_bars(headers.iter()),
        build_hline(headers, 0, headers.len()),
    ];

    table.extend(records.iter().map(|record| join_with_bars(record.iter())));
    table
}
