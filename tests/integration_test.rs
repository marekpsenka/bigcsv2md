#[test]
fn test_lipsum_simple() {
    use bigcsv2md::common::read_headers_records;
    use bigcsv2md::simple::to_md_table_simple;
    let input = include_str!("data/lipsum.csv");

    let (headers, records) = read_headers_records(input.as_bytes()).expect("Read successful");
    let mut result_iter = to_md_table_simple(&headers, &records).into_iter();

    let output = include_str!("data/lipsum.md");
    let mut output_lines = output.lines();

    while let Some(line) = output_lines.next() {
        let result_line = result_iter.next();
        assert!(result_line.is_some());

        assert!(line == result_line.unwrap().as_str())
    }
}