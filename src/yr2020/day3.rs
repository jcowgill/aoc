use nalgebra::DMatrix;

fn parse_grid(input: &str) -> DMatrix<bool> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    DMatrix::from_row_iterator(
        height,
        width,
        input
            .lines()
            .flat_map(|line| line.chars().map(|c| c == '#')),
    )
}

fn count_trees(grid: &DMatrix<bool>, dx: usize, dy: usize) -> usize {
    (0..grid.nrows())
        .step_by(dy)
        .filter(|&y| {
            let x = (y * dx / dy) % grid.ncols();
            grid[(y, x)]
        })
        .count()
}

pub fn star1(input: &str) -> String {
    count_trees(&parse_grid(input), 3, 1).to_string()
}

pub fn star2(input: &str) -> String {
    let grid = parse_grid(input);
    (count_trees(&grid, 1, 1)
        * count_trees(&grid, 3, 1)
        * count_trees(&grid, 5, 1)
        * count_trees(&grid, 7, 1)
        * count_trees(&grid, 1, 2))
    .to_string()
}
