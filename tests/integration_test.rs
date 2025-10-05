mod common {
    pub static LIPSUM_CSV: &str = include_str!("data/lipsum.csv");
}

#[test]
fn test_lipsum_simple() {
    use bigcsv2md::common::read_headers_records;
    use bigcsv2md::simple::to_md_table_simple;
    use common::LIPSUM_CSV;

    let (headers, records) = read_headers_records(LIPSUM_CSV.as_bytes()).expect("Read successful");
    let mut result_iter = to_md_table_simple(&headers, &records).into_iter();

    let output = include_str!("data/lipsum.md");

    for line in output.lines() {
        let result_line = result_iter.next();
        assert!(result_line.is_some());

        assert!(line == result_line.unwrap().as_str())
    }
}

#[test]
fn test_lipsum_csplit3() {
    use bigcsv2md::common::read_headers_records;
    use bigcsv2md::csplit::to_md_tables_csplit;
    use common::LIPSUM_CSV;
    use itertools::Itertools;

    let (headers, records) = read_headers_records(LIPSUM_CSV.as_bytes()).expect("Read successful");
    let mut result_iter = to_md_tables_csplit(&headers, &records, 3, false).into_iter();

    let output = include_str!("data/lipsum_csplit.md");

    for (nonempty, chunk) in &output.lines().chunk_by(|line| !((*line).is_empty())) {
        if !nonempty {
            continue;
        }

        let result_lines = result_iter.next();
        assert!(result_lines.is_some());
        let mut result_lines = result_lines.unwrap().into_iter();

        for line in chunk {
            let result_line = result_lines.next();
            assert!(result_line.is_some());

            assert!(line == result_line.unwrap().as_str())
        }
    }
}
