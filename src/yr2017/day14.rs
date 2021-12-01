use crate::yr2017::day10::{knot_hash, KnotHashResult};

/// The size of the grid (in both dimensions)
const GRID_SIZE: u8 = 128;

/// Calculates the disk grid from the given input key
///  The returned vector always contains 128 elements
fn get_grid(key: &str) -> Vec<KnotHashResult> {
    (0..128)
        .map(|row| knot_hash(format!("{}-{}", key, row).bytes()))
        .collect()
}

/// Find number of bits set in defrag disk
pub fn star1(input: &str) -> String {
    get_grid(input)
        .iter()
        .map(|row| row.iter().map(|b| b.count_ones()).sum::<u32>())
        .sum::<u32>()
        .to_string()
}

/// Returns the value at a given point
fn get_point(grid: &[KnotHashResult], (x, y): (u8, u8)) -> bool {
    assert!(y < GRID_SIZE);
    assert!(x < GRID_SIZE);

    (grid[y as usize][(x / 8) as usize] & (1 << (7 - x % 8))) != 0
}

/// Clears the value at a given point
fn clear_point(grid: &mut [KnotHashResult], (x, y): (u8, u8)) {
    assert!(y < GRID_SIZE);
    assert!(x < GRID_SIZE);

    grid[y as usize][(x / 8) as usize] &= !(1 << (7 - x % 8));
}

/// Consumes the region containing the given point
fn consume_region(grid: &mut [KnotHashResult], point: (u8, u8)) {
    // Only continue if point is actually set
    if get_point(grid, point) {
        // Erase point and recurse to 4 surrounding points
        clear_point(grid, point);
        if point.0 != 0 {
            consume_region(grid, (point.0 - 1, point.1))
        };
        if point.1 != 0 {
            consume_region(grid, (point.0, point.1 - 1))
        };
        if point.0 != (GRID_SIZE - 1) {
            consume_region(grid, (point.0 + 1, point.1))
        };
        if point.1 != (GRID_SIZE - 1) {
            consume_region(grid, (point.0, point.1 + 1))
        };
    }
}

/// Finds any point in the grid which is in use
fn find_used_point(grid: &[KnotHashResult]) -> Option<(u8, u8)> {
    for y in 0..GRID_SIZE {
        for byte in 0..(GRID_SIZE / 8) {
            let byte_value = grid[y as usize][byte as usize];
            if byte_value != 0 {
                return Some((byte * 8 + byte_value.leading_zeros() as u8, y));
            }
        }
    }

    None
}

/// Counts the number of regions in a grid
fn count_regions(mut grid: Vec<KnotHashResult>) -> u32 {
    let mut regions = 0;
    while let Some(point) = find_used_point(&grid) {
        consume_region(&mut grid, point);
        regions += 1;
    }
    regions
}

/// Find total number of regions in defrag disk
pub fn star2(input: &str) -> String {
    count_regions(get_grid(input)).to_string()
}
