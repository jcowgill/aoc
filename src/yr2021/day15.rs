use std::cmp::Reverse;

use crate::direction::Direction;
use nalgebra::Vector2;

type Position = Vector2<i32>;

#[derive(Debug, Clone)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
}

impl<T: Copy> Grid<T> {
    fn pos_to_index(&self, pos: Position) -> Option<usize> {
        if let (Ok(x), Ok(y)) = (usize::try_from(pos.x), usize::try_from(pos.y))
            && x < self.width
        {
            return Some(x + y * self.width);
        }

        None
    }

    fn get(&self, pos: Position) -> Option<T> {
        self.pos_to_index(pos)
            .and_then(|i| self.data.get(i).copied())
    }

    fn get_mut(&mut self, pos: Position) -> Option<&mut T> {
        self.pos_to_index(pos).and_then(|i| self.data.get_mut(i))
    }

    fn surrounding(&self, pos: Position) -> impl Iterator<Item = (Position, T)> + '_ {
        Direction::iter().filter_map(move |d| {
            let new_pos = pos + d.to_vec();
            self.get(new_pos).map(|v| (new_pos, v))
        })
    }
}

fn parse_input(input: &str) -> Grid<u32> {
    let mut lines = input.lines().peekable();
    let width = lines.peek().unwrap().trim().len();
    Grid {
        data: lines
            .flat_map(|l| l.trim().chars().map(|c| c.to_digit(10).unwrap()))
            .collect(),
        width,
    }
}

fn shortest_path(grid: &Grid<u32>) -> u32 {
    let start = Vector2::zeros();
    let mut cost_grid = Grid {
        data: vec![u32::MAX; grid.width * grid.width],
        width: grid.width,
    };

    let mut open = vec![(0, start)];
    let key_fn = |&(c, p): &(u32, Position)| Reverse((c, p.x, p.y));
    *cost_grid.get_mut(start).unwrap() = 0;

    while let Some((my_cost, pos)) = open.pop() {
        for (adj, adj_immediate_cost) in grid.surrounding(pos) {
            let new_adj_cost = my_cost + adj_immediate_cost;
            let adj_cost_ptr = cost_grid.get_mut(adj).unwrap();
            if new_adj_cost < *adj_cost_ptr {
                let new_value = (new_adj_cost, adj);
                let insert_pos = open
                    .binary_search_by_key(&key_fn(&new_value), key_fn)
                    .unwrap_err();

                let maybe_remove_pos = if *adj_cost_ptr == u32::MAX {
                    None
                } else {
                    open[..insert_pos]
                        .binary_search_by_key(&key_fn(&new_value), key_fn)
                        .ok()
                };

                if let Some(remove_pos) = maybe_remove_pos {
                    // If this node already exists in the open array,
                    // rotate the elements and update the new value
                    open[remove_pos..insert_pos].rotate_left(1);
                    open[insert_pos - 1] = new_value;
                } else {
                    // Insert node in the correct place
                    open.insert(insert_pos, new_value);
                }

                *adj_cost_ptr = new_adj_cost;
            }
        }
    }

    let end = Vector2::repeat((grid.width - 1) as i32);
    cost_grid.get(end).unwrap()
}

pub fn star1(input: &str) -> String {
    shortest_path(&parse_input(input)).to_string()
}

pub fn star2(input: &str) -> String {
    let small_grid = parse_input(input);
    let large_grid_width = small_grid.width * 5;
    let large_grid = Grid {
        data: (0..large_grid_width * large_grid_width)
            .map(|off| {
                let y = off / large_grid_width;
                let x = off % large_grid_width;
                let small_y = (y % small_grid.width) as i32;
                let small_x = (x % small_grid.width) as i32;
                let tile_y = (y / small_grid.width) as u32;
                let tile_x = (x / small_grid.width) as u32;

                (small_grid.get(Vector2::new(small_x, small_y)).unwrap() + tile_y + tile_x - 1) % 9
                    + 1
            })
            .collect(),
        width: small_grid.width * 5,
    };
    shortest_path(&large_grid).to_string()
}
