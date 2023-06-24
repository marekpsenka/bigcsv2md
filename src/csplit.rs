use itertools::Itertools;
use std::fmt::Display;

#[derive(Debug)]
struct Guides {
    ncols: usize,
    csplit: usize,
    div: usize,
    div_ceil: usize,
    rem: usize,
}

impl Guides {
    fn new(ncols: usize, csplit: usize) -> Guides {
        Guides {
            ncols,
            csplit,
            div: ncols / csplit,
            div_ceil: (ncols + csplit - 1) / csplit,
            rem: ncols % csplit,
        }
    }
}

fn join_with_bars<T>(mut iter: impl Iterator<Item = T>) -> String
where
    T: Display,
{
    let mut joined = iter.join("|");
    joined.insert(0, '|');
    joined.insert(joined.len(), '|');
    joined
}

fn build_hline(rec: &csv::StringRecord, col_from: usize, col_to: usize) -> String {
    let to_join = (col_from..col_to).map(|icol| {
        let range = rec.range(icol).expect("Valid index");
        "-".repeat(range.len().max(1))
    });
    join_with_bars(to_join)
}

fn record_columns_to_md(rec: &csv::StringRecord, col_from: usize, col_to: usize) -> String {
    let to_join = (col_from..col_to).map(|icol| {
        let range = rec.range(icol).expect("Valid index");
        &rec.as_slice()[range]
    });
    join_with_bars(to_join)
}

fn table_columns_to_md(
    headers: &csv::StringRecord,
    records: &[csv::StringRecord],
    col_from: usize,
    col_to: usize,
) -> Vec<String> {
    let mut this_table = vec![
        record_columns_to_md(headers, col_from, col_to),
        build_hline(headers, col_from, col_to),
    ];

    this_table.extend(
        records
            .iter()
            .map(|rec| record_columns_to_md(rec, col_from, col_to)),
    );
    this_table
}

pub fn to_md_tables_csplit(
    headers: &csv::StringRecord,
    records: &[csv::StringRecord],
    csplit: usize,
) -> Vec<Vec<String>> {
    let ncols = headers.len();
    let guides = Guides::new(ncols, csplit);
    let mut tables = (0..guides.div)
        .map(|isplit| {
            let col_from = isplit * guides.csplit;
            let col_to = (isplit + 1) * guides.csplit;
            table_columns_to_md(headers, records, col_from, col_to)
        })
        .collect::<Vec<Vec<String>>>();
    if guides.div != guides.div_ceil {
        tables.push(table_columns_to_md(
            headers,
            records,
            guides.div * guides.csplit,
            guides.div * guides.csplit + guides.rem,
        ));
    }
    tables
}
