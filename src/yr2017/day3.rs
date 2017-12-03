fn grid_distance(value: i32) -> i32 {
    assert!(value >= 1);

    // Special case 1
    if value == 1 { return 0 };

    // Find ring number and last value in previous ring (always square)
    let ring_number = ((((value - 1) as f64).sqrt() + 1.0) / 2.0).floor() as i32;
    let last_square = (2 * ring_number - 1).pow(2);

    // Find which side this value is on (0 = right, 1 = top, 2 = left, 3 = bottom)
    let side = (value - last_square - 1) / (2 * ring_number);
    assert!(side >= 0 && side < 4);

    // Find mid point of side (where grid distance = ring number)
    let midpoint = last_square + 2 * ring_number * side + ring_number;

    // Calculate final distance
    ring_number + (value - midpoint).abs()
}

/// Manhattan distance from infinite spiral grid value to center
pub fn star1(input: &str) -> String {
    grid_distance(input.parse().unwrap()).to_string()
}
