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
