use cartesian_product;

/// Calculate spreadsheet checksum
pub fn star1(input: &str) -> String {
    let line_checksum = |line: &str| {
        let values: Vec<i32> =
            line.split_whitespace().map(|value| value.parse().unwrap()).collect();
        values.iter().max().unwrap_or(&0) - values.iter().min().unwrap_or(&0)
    };

    let sum: i32 = input.lines().map(line_checksum).sum();
    sum.to_string()
}

/// Calculate spreadsheet checksum using evenly divisible numbers
pub fn star2(input: &str) -> String {
    let line_checksum = |line: &str| {
        let values: Vec<i32> =
            line.split_whitespace().map(|value| value.parse().unwrap()).collect();

        for (a, b) in cartesian_product(&values, &values) {
            if a != b && a % b == 0 {
                return a / b;
            }
        }

        panic!("input line contains no valid pairs: {:?}", values)
    };

    let sum: i32 = input.lines().map(line_checksum).sum();
    sum.to_string()
}
