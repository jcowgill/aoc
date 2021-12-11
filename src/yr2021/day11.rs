type Grid = [u8; 100];

fn parse_grid(input: &str) -> Grid {
    input
        .chars()
        .flat_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

/// Returns an iterator over all the valid points around a point in the grid
fn surrounding(index: usize) -> impl Iterator<Item = usize> {
    const OFFSETS: [(i8, i8); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let y = (index / 10) as i8;
    let x = (index % 10) as i8;

    OFFSETS
        .iter()
        .map(move |(off_x, off_y)| (x + off_x, y + off_y))
        .filter(|(new_x, new_y)| (0..10).contains(new_x) && (0..10).contains(new_y))
        .map(|(new_x, new_y)| (new_x + new_y * 10) as usize)
}

/// Give one energy to the given cell and flash if needed
fn give_energy(grid: &mut Grid, closed: &mut Vec<usize>, index: usize) {
    grid[index] += 1;
    if grid[index] == 10 {
        closed.push(index);
        for i in surrounding(index) {
            give_energy(grid, closed, i);
        }
    }
}

/// Steps the grid one time step and returns the number of flashes which occured
fn step_grid(grid: &mut Grid) -> usize {
    let mut closed = Vec::new();

    // Give one energy to every cell and handle flashes
    for i in 0..grid.len() {
        give_energy(grid, &mut closed, i);
    }

    // Set closed cells to zero
    let flash_count = closed.len();
    for i in closed {
        grid[i] = 0;
    }

    flash_count
}

pub fn star1(input: &str) -> String {
    let mut grid = parse_grid(input);
    (0..100)
        .map(|_| step_grid(&mut grid))
        .sum::<usize>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    let mut grid = parse_grid(input);
    ((0..).find(|_| step_grid(&mut grid) == grid.len()).unwrap() + 1).to_string()
}
