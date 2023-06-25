use crate::{build_hline, join_with_bars};

#[derive(Debug)]
struct Guides {
    coffset: usize,
    csplit: usize,
    div: usize,
    div_ceil: usize,
    rem: usize,
}

impl Guides {
    fn new(ncols: usize, csplit: usize, rheaders: bool) -> Guides {
        if rheaders {
            Guides {
                coffset: 1,
                csplit,
                div: (ncols - 1) / csplit,
                div_ceil: (ncols + csplit - 2) / csplit,
                rem: (ncols - 1) % csplit
            }
        }
        else {
            Guides {
                coffset: 0,
                csplit,
                div: ncols / csplit,
                div_ceil: (ncols + csplit - 1) / csplit,
                rem: ncols % csplit
            }
        }
    }
}



fn record_column_to_str(rec: &csv::StringRecord, icol: usize) -> &str {
    let range = rec.range(icol).expect("Valid Index");
    &rec.as_slice()[range]
}

fn record_columns_to_md(rec: &csv::StringRecord, col_from: usize, col_to: usize) -> String {
    let to_join = (col_from..col_to).map(|icol| record_column_to_str(rec, icol));
    join_with_bars(to_join)
}

fn table_columns_to_md(
    headers: &csv::StringRecord,
    records: &[csv::StringRecord],
    col_from: usize,
    col_to: usize,
    maybe_rheaders: Option<&[String]>
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

    if let Some(rheaders) = maybe_rheaders {
        this_table
            .iter_mut()
            .zip(rheaders.iter())
            .for_each(|(s, rh)| {
                s.insert_str(0, rh.as_str());
            })
    }

    this_table
}

pub fn to_md_tables_csplit(
    headers: &csv::StringRecord,
    records: &[csv::StringRecord],
    csplit: usize,
    rheaders: bool
) -> Vec<Vec<String>> {
    let ncols = headers.len();
    let guides = Guides::new(ncols, csplit, rheaders);

    let maybe_rheaders = if rheaders {
        let mut rheaders = vec!["| ".to_owned(), "|-".to_owned()]; // TODO: Not ideal
        rheaders.extend(records.iter().map(|rec| {
            let mut col = record_column_to_str(rec, 0).to_owned();
            col.insert(0, '|');
            col
        }));
        Some(rheaders)
    }
    else { None };

    let mut tables = (0..guides.div)
        .map(|isplit| {
            let col_from = guides.coffset + isplit * guides.csplit;
            let col_to = guides.coffset + (isplit + 1) * guides.csplit;
            table_columns_to_md(headers, records, col_from, col_to, maybe_rheaders.as_deref())
        })
        .collect::<Vec<Vec<String>>>();
    if guides.div != guides.div_ceil {
        tables.push(table_columns_to_md(
            headers,
            records,
            guides.coffset + guides.div * guides.csplit,
            guides.coffset + guides.div * guides.csplit + guides.rem,
            maybe_rheaders.as_deref()
        ));
    }
    tables
}
