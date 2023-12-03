use itertools::Itertools;
use nalgebra::DMatrix;

fn parse_input(input: &str) -> DMatrix<u8> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    DMatrix::from_iterator(height, width, input.lines().flat_map(|line| line.bytes()))
}

fn find_num(grid: &DMatrix<u8>, x: i32, y: i32) -> Option<(usize, usize)> {
    let mut xus = x as usize;
    let yus = y as usize;
    if grid
        .get((xus, yus))
        .filter(|&v| v.is_ascii_digit())
        .is_some()
    {
        while xus > 0 && grid[(xus - 1, yus)].is_ascii_digit() {
            xus -= 1;
        }
        Some((xus, yus))
    } else {
        None
    }
}

fn iterate_grid_nums(
    grid: &DMatrix<u8>,
) -> impl Iterator<Item = (u8, impl Iterator<Item = (usize, usize)> + '_)> + '_ {
    (0..grid.nrows()).flat_map(move |y| {
        (0..grid.ncols()).map(move |x| {
            (
                grid[(x, y)],
                (-1..=1).flat_map(move |oy| {
                    (-1..=1).filter_map(move |ox| find_num(grid, x as i32 + ox, y as i32 + oy))
                }),
            )
        })
    })
}

fn extract_num(grid: &DMatrix<u8>, (mut x, y): (usize, usize)) -> u32 {
    let mut value = 0;
    while let Some(d) = grid.get((x, y)).filter(|&d| d.is_ascii_digit()) {
        value = value * 10 + u32::from(d - b'0');
        x += 1;
    }
    value
}

pub fn star1(input: &str) -> String {
    let grid = parse_input(input);
    iterate_grid_nums(&grid)
        .filter(|(v, _)| *v != b'.' && !v.is_ascii_digit())
        .flat_map(|(_, ns)| ns)
        .sorted_unstable()
        .dedup()
        .map(|pos| extract_num(&grid, pos))
        .sum::<u32>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    let grid = parse_input(input);
    iterate_grid_nums(&grid)
        .filter(|(v, _)| *v == b'*')
        .map(|(_, ns)| ns.dedup().collect_vec())
        .filter(|ns| ns.len() == 2)
        .map(|ns| {
            ns.into_iter()
                .map(|pos| extract_num(&grid, pos))
                .product::<u32>()
        })
        .sum::<u32>()
        .to_string()
}
