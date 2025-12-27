use std::cmp;

/// Returns the ring side number a point is on
///  0 = Right
///  1 = Up
///  2 = Left
///  3 = Down
fn xy_to_side((x, y): (i32, i32)) -> i32 {
    if x >= 0 && y >= 0 {
        i32::from(x < y)
    } else if x < 0 && y >= 0 {
        if y >= -x { 1 } else { 2 }
    } else if x < 0 && y < 0 {
        if x <= y { 2 } else { 3 }
    } else if y <= -x {
        3
    } else {
        0
    }
}

/// Gets the spiral grid value at a specific point
fn spiral_grid_value(point: (i32, i32)) -> i32 {
    // Determine ring number, last value in previous ring and side number
    let (x, y) = point;
    let ring_number = cmp::max(x.abs(), y.abs());
    let last_square = (2 * ring_number - 1).pow(2);
    let side_number = xy_to_side(point);

    // Find the distance along our side
    let side_distance = match side_number {
        0 => ring_number + y,
        1 => ring_number - x,
        2 => ring_number - y,
        3 => ring_number + x,
        _ => panic!("invalid side number"),
    };

    // Return calculated value
    last_square + 2 * ring_number * side_number + side_distance
}

/// Gets the x,y coordinates containing the given spiral value
fn spiral_grid_xy(value: i32) -> (i32, i32) {
    assert!(value >= 1);

    // Special case 1
    if value == 1 {
        return (0, 0);
    };

    // Determine ring number, last value in previous ring and side number
    let ring_number = ((((value - 1) as f64).sqrt() + 1.0) / 2.0).floor() as i32;
    let last_square = (2 * ring_number - 1).pow(2);
    let ring_distance = value - last_square - 1;
    let side_number = ring_distance / (2 * ring_number);

    // Find side distance and calculate final value
    let side_distance_p1 = ring_distance % (2 * ring_number) + 1;
    match side_number {
        0 => (ring_number, side_distance_p1 - ring_number),
        1 => (ring_number - side_distance_p1, ring_number),
        2 => (-ring_number, ring_number - side_distance_p1),
        3 => (side_distance_p1 - ring_number, -ring_number),
        _ => panic!("invalid side number"),
    }
}

/// Manhattan distance from infinite spiral grid value to center
pub fn star1(input: &str) -> String {
    let (x, y) = spiral_grid_xy(input.parse().unwrap());
    (x.abs() + y.abs()).to_string()
}

/// Sum numbers around points in spiral order, return first value larger than input
pub fn star2(input: &str) -> String {
    /// Returns value at a point in the spreadsheet
    fn value_at(spreadsheet: &[i32], x: i32, y: i32) -> i32 {
        *spreadsheet
            .get(spiral_grid_value((x, y)) as usize - 1)
            .unwrap_or(&0)
    }

    // The spreadsheet is represented as a growing vector where the indexes spiral around
    //  The x,y values for an index are calculated using spiral_grid_xy(index)
    let mut spreadsheet = vec![1];

    // Continue until last value is greater than input
    let stop_value: i32 = input.parse().unwrap();
    assert!(stop_value >= 1);

    while *spreadsheet.last().unwrap() < stop_value {
        let (x, y) = spiral_grid_xy(spreadsheet.len() as i32 + 1);
        let new_value = value_at(&spreadsheet, x - 1, y - 1)
            + value_at(&spreadsheet, x - 1, y)
            + value_at(&spreadsheet, x - 1, y + 1)
            + value_at(&spreadsheet, x, y - 1)
            + value_at(&spreadsheet, x, y + 1)
            + value_at(&spreadsheet, x + 1, y - 1)
            + value_at(&spreadsheet, x + 1, y)
            + value_at(&spreadsheet, x + 1, y + 1);
        spreadsheet.push(new_value);
    }

    spreadsheet.last().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Structure to hold test cases
    struct TestEntry {
        point: (i32, i32),
        value: i32,
        side: i32,
    }

    /// Data for unit tests
    const UNIT_TEST_DATA: [TestEntry; 17] = [
        TestEntry {
            point: (0, 0),
            value: 1,
            side: 0,
        },
        TestEntry {
            point: (2, -1),
            value: 10,
            side: 0,
        },
        TestEntry {
            point: (2, 0),
            value: 11,
            side: 0,
        },
        TestEntry {
            point: (2, 1),
            value: 12,
            side: 0,
        },
        TestEntry {
            point: (2, 2),
            value: 13,
            side: 0,
        },
        TestEntry {
            point: (1, 2),
            value: 14,
            side: 1,
        },
        TestEntry {
            point: (0, 2),
            value: 15,
            side: 1,
        },
        TestEntry {
            point: (-1, 2),
            value: 16,
            side: 1,
        },
        TestEntry {
            point: (-2, 2),
            value: 17,
            side: 1,
        },
        TestEntry {
            point: (-2, 1),
            value: 18,
            side: 2,
        },
        TestEntry {
            point: (-2, 0),
            value: 19,
            side: 2,
        },
        TestEntry {
            point: (-2, -1),
            value: 20,
            side: 2,
        },
        TestEntry {
            point: (-2, -2),
            value: 21,
            side: 2,
        },
        TestEntry {
            point: (-1, -2),
            value: 22,
            side: 3,
        },
        TestEntry {
            point: (0, -2),
            value: 23,
            side: 3,
        },
        TestEntry {
            point: (1, -2),
            value: 24,
            side: 3,
        },
        TestEntry {
            point: (2, -2),
            value: 25,
            side: 3,
        },
    ];

    #[test]
    fn test_xy_to_side() {
        for entry in UNIT_TEST_DATA.iter() {
            println!("trying {:?}", entry.point);
            assert_eq!(xy_to_side(entry.point), entry.side);
        }
    }

    #[test]
    fn test_spiral_grid_value() {
        for entry in UNIT_TEST_DATA.iter() {
            println!("trying {:?}", entry.point);
            assert_eq!(spiral_grid_value(entry.point), entry.value);
        }
    }

    #[test]
    fn test_spiral_grid_xy() {
        for entry in UNIT_TEST_DATA.iter() {
            println!("trying {:?}", entry.value);
            assert_eq!(spiral_grid_xy(entry.value), entry.point);
        }
    }
}
