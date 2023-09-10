use itertools::Itertools;
use nalgebra::DMatrix;

fn parse_grid(input: &str) -> DMatrix<u8> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().trim().len();
    DMatrix::from_row_iterator(
        rows,
        cols,
        input
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .map(|c| c as u8 - b'0'),
    )
}

fn process_visible<'a, 'b>(
    values: impl Iterator<Item = &'a u8>,
    visible: impl Iterator<Item = &'b mut bool>,
) {
    let mut best = None;
    for (&h, v) in values.zip(visible) {
        if best.map(|b| h > b).unwrap_or(true) {
            *v = true;
            best = Some(h);
        }
    }
}

pub fn star1(input: &str) -> String {
    let grid = parse_grid(input);
    let mut visible = DMatrix::repeat(grid.nrows(), grid.ncols(), false);

    for (grid_row, mut visible_row) in grid.row_iter().zip(visible.row_iter_mut()) {
        process_visible(grid_row.iter(), visible_row.iter_mut());
        process_visible(grid_row.iter().rev(), visible_row.iter_mut().rev());
    }

    for (grid_col, mut visible_col) in grid.column_iter().zip(visible.column_iter_mut()) {
        process_visible(grid_col.iter(), visible_col.iter_mut());
        process_visible(grid_col.iter().rev(), visible_col.iter_mut().rev());
    }

    visible.into_iter().filter(|&&v| v).count().to_string()
}

fn count_trees<'a>(mut heights: impl Iterator<Item = &'a u8>) -> usize {
    let me = *heights.next().unwrap();
    let mut count = 0;
    for &h in heights {
        count += 1;
        if h >= me {
            break;
        }
    }
    count
}

pub fn star2(input: &str) -> String {
    let grid = parse_grid(input);

    (0..grid.nrows())
        .cartesian_product(0..grid.ncols())
        .map(|(y, x)| {
            let left = count_trees(grid.view_range(y, 0..=x).iter().rev());
            let right = count_trees(grid.view_range(y, x..).iter());
            let up = count_trees(grid.view_range(0..=y, x).iter().rev());
            let down = count_trees(grid.view_range(y.., x).iter());
            left * right * up * down
        })
        .max()
        .unwrap_or(0)
        .to_string()
}
