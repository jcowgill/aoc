use crate::direction::Direction;
use nalgebra::Vector2;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Tile {
    OffMap,
    Empty,
    Wall,
}

fn parse_grid(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    _ => Tile::OffMap,
                })
                .collect()
        })
        .collect()
}

fn parse_instructions(
    instrs: &str,
) -> impl Iterator<Item = (usize, fn(Direction) -> Direction)> + '_ {
    instrs.split_inclusive(&['L', 'R']).map(|s| {
        let num = s.trim_end_matches(&['L', 'R']).parse().unwrap();
        let rotation: fn(Direction) -> Direction = if s.ends_with('L') {
            |d| d.anticlockwise()
        } else if s.ends_with('R') {
            |d| d.clockwise()
        } else {
            |d| d
        };

        (num, rotation)
    })
}

fn grid_get(grid: &[Vec<Tile>], pos: Vector2<i32>) -> Tile {
    usize::try_from(pos.y)
        .ok()
        .and_then(|y| {
            grid.get(y).and_then(|row| {
                usize::try_from(pos.x)
                    .ok()
                    .and_then(|x| row.get(x).copied())
            })
        })
        .unwrap_or(Tile::OffMap)
}

fn solve(
    input: &str,
    wrap_fn: impl Fn(&[Vec<Tile>], Direction, Vector2<i32>) -> (Direction, Vector2<i32>),
) -> String {
    let (grid_str, instr_str) = input.split_once("\n\n").unwrap();
    let grid = parse_grid(grid_str);

    let mut pos = Vector2::new(
        grid[0].iter().position(|&t| t == Tile::Empty).unwrap() as i32,
        0,
    );
    let mut dir = Direction::East;

    for (len, post_rotation) in parse_instructions(instr_str) {
        for _ in 0..len {
            let try_pos = pos + dir.to_vec_neg();

            match grid_get(&grid, try_pos) {
                Tile::Empty => pos = try_pos,
                Tile::Wall => break,
                Tile::OffMap => {
                    let (wrap_dir, wrap_pos) = wrap_fn(&grid, dir, pos);
                    if grid_get(&grid, wrap_pos) == Tile::Wall {
                        break;
                    }

                    dir = wrap_dir;
                    pos = wrap_pos;
                }
            };
        }

        dir = post_rotation(dir);
    }

    let facing = match dir {
        Direction::East => 0,
        Direction::South => 1,
        Direction::West => 2,
        Direction::North => 3,
    };

    (1000 * (pos.y + 1) + 4 * (pos.x + 1) + facing).to_string()
}

fn wrap_position(grid: &[Vec<Tile>], dir: Direction, pos: Vector2<i32>) -> Vector2<i32> {
    (0..)
        .map(|off| pos + dir.reverse().to_vec_neg() * off)
        .take_while(|&pos| grid_get(grid, pos) != Tile::OffMap)
        .last()
        .unwrap()
}

pub fn star1(input: &str) -> String {
    solve(input, |grid, dir, pos| (dir, wrap_position(grid, dir, pos)))
}

fn wrap_cube_small(dir: Direction, pos: Vector2<i32>) -> (Direction, Vector2<i32>) {
    let s = 4;
    let block = (pos - dir.to_vec_neg()) / s;
    let pmod = (pos - dir.to_vec_neg()).map(|v| v % s);

    let (dir, x, y) = match (dir, block.x, block.y) {
        (Direction::North, 0, 1) => (Direction::South, 3 * s - pmod.x - 1, 0),
        (Direction::North, 2, 0) => (Direction::South, s - pmod.x - 1, s),

        (Direction::North, 1, 1) => (Direction::East, 2 * s, pmod.x),
        (Direction::West, 2, 0) => (Direction::South, s + pmod.y, s),

        (Direction::North, 3, 2) => (Direction::West, 3 * s - 1, 2 * s - pmod.x - 1),
        (Direction::East, 2, 1) => (Direction::South, 4 * s - pmod.y - 1, 2 * s),

        (Direction::South, 0, 1) => (Direction::North, 3 * s - pmod.x - 1, 3 * s - 1),
        (Direction::South, 2, 2) => (Direction::North, s - pmod.x - 1, 2 * s - 1),

        (Direction::South, 1, 1) => (Direction::East, 2 * s, 2 * s + pmod.x),
        (Direction::West, 2, 2) => (Direction::North, s + pmod.y, 2 * s - 1),

        (Direction::South, 3, 2) => (Direction::East, 0, 2 * s - pmod.x - 1),
        (Direction::West, 0, 1) => (Direction::North, 4 * s - pmod.y - 1, 3 * s - 1),

        (Direction::East, 2, 0) => (Direction::West, 4 * s - 1, 3 * s - pmod.y - 1),
        (Direction::East, 3, 2) => (Direction::West, 3 * s - 1, s - pmod.y - 1),

        _ => panic!("unhandled wrap"),
    };

    (dir, Vector2::new(x, y))
}

fn wrap_cube_big(dir: Direction, pos: Vector2<i32>) -> (Direction, Vector2<i32>) {
    let s = 50;
    let block = (pos - dir.to_vec_neg()) / s;
    let pmod = (pos - dir.to_vec_neg()).map(|v| v % s);

    let (dir, x, y) = match (dir, block.x, block.y) {
        (Direction::North, 0, 2) => (Direction::East, s, s + pmod.x),
        (Direction::West, 1, 1) => (Direction::South, pmod.y, 2 * s),

        (Direction::North, 1, 0) => (Direction::East, 0, 3 * s + pmod.x),
        (Direction::West, 0, 3) => (Direction::South, s + pmod.y, 0),

        (Direction::North, 2, 0) => (Direction::North, pmod.x, 4 * s - 1),
        (Direction::South, 0, 3) => (Direction::South, 2 * s + pmod.x, 0),

        (Direction::South, 1, 2) => (Direction::West, s - 1, 3 * s + pmod.x),
        (Direction::East, 0, 3) => (Direction::North, s + pmod.y, 3 * s - 1),

        (Direction::South, 2, 0) => (Direction::West, 2 * s - 1, s + pmod.x),
        (Direction::East, 1, 1) => (Direction::North, 2 * s + pmod.y, s - 1),

        (Direction::East, 2, 0) => (Direction::West, 2 * s - 1, 3 * s - pmod.y - 1),
        (Direction::East, 1, 2) => (Direction::West, 3 * s - 1, s - pmod.y - 1),

        (Direction::West, 1, 0) => (Direction::East, 0, 3 * s - pmod.y - 1),
        (Direction::West, 0, 2) => (Direction::East, s, s - pmod.y - 1),

        _ => panic!("unhandled wrap"),
    };

    (dir, Vector2::new(x, y))
}

pub fn star2(input: &str) -> String {
    solve(input, |grid, dir, pos| {
        if grid.len() > 100 {
            wrap_cube_big(dir, pos)
        } else {
            wrap_cube_small(dir, pos)
        }
    })
}
