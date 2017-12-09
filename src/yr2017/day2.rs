use cartesian_product;

/// Implements a generic spreadsheet checksum function
///  Parses the input data and passes the vector of values on each line to line_func,
///  then sums all the lines up and returns the result
fn spreadsheet_checksum<F>(input: &str, line_value: F) -> String where F: Fn(&Vec<i32>) -> i32 {
    let line_checksum = |line: &str| {
        let values: Vec<i32> =
            line.split_whitespace().map(|value| value.parse().unwrap()).collect();
        line_value(&values)
    };

    let sum: i32 = input.lines().map(line_checksum).sum();
    sum.to_string()
}

/// Calculate spreadsheet checksum
pub fn star1(input: &str) -> String {
    spreadsheet_checksum(input, |values| {
        values.iter().max().unwrap_or(&0) - values.iter().min().unwrap_or(&0)
    })
}

/// Calculate spreadsheet checksum using evenly divisible numbers
pub fn star2(input: &str) -> String {
    spreadsheet_checksum(input, |values| {
        for (a, b) in cartesian_product(values.iter(), values.iter()) {
            if a != b && a % b == 0 {
                return a / b;
            }
        }

        panic!("input line contains no valid pairs: {:?}", values)
    })
}
