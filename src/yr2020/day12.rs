use nalgebra::{Matrix2, Vector2};

use crate::direction::Direction;
use crate::vector::VectorExt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct State {
    pos: Vector2<i32>,
    way: Vector2<i32>,
}

#[derive(Clone, Copy, Debug)]
enum Transition {
    Move(Vector2<i32>),
    Forward(i32),
    Rotate(Matrix2<i32>),
}

impl State {
    fn apply(self, transition: Transition, move_waypoint: bool) -> State {
        match (transition, move_waypoint) {
            (Transition::Move(v), false) => State {
                pos: self.pos + v,
                way: self.way,
            },
            (Transition::Move(v), true) => State {
                pos: self.pos,
                way: self.way + v,
            },
            (Transition::Forward(a), _) => State {
                pos: self.pos + self.way * a,
                way: self.way,
            },
            (Transition::Rotate(m), _) => State {
                pos: self.pos,
                way: m * self.way,
            },
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Transition> + '_ {
    input.lines().map(|l| {
        let mut cs = l.chars();
        let mut ty = cs.next().unwrap();
        let mut amount = cs.as_str().parse::<i32>().unwrap();

        if ty == 'L' {
            ty = 'R';
            amount = 360 - amount;
        }

        match ty {
            'N' => Transition::Move(Direction::North.to_vec() * amount),
            'S' => Transition::Move(Direction::South.to_vec() * amount),
            'E' => Transition::Move(Direction::East.to_vec() * amount),
            'W' => Transition::Move(Direction::West.to_vec() * amount),
            'R' if amount == 90 => Transition::Rotate(Matrix2::new(0, 1, -1, 0)),
            'R' if amount == 180 => Transition::Rotate(Matrix2::new(-1, 0, 0, -1)),
            'R' if amount == 270 => Transition::Rotate(Matrix2::new(0, -1, 1, 0)),
            'F' => Transition::Forward(amount),
            _ => panic!("invalid instruction"),
        }
    })
}

fn run(input: &str, move_waypoint: bool, start: State) -> String {
    parse_input(input)
        .fold(start, |s, t| s.apply(t, move_waypoint))
        .pos
        .taxicab_norm()
        .to_string()
}

pub fn star1(input: &str) -> String {
    run(
        input,
        false,
        State {
            pos: Vector2::zeros(),
            way: Vector2::new(1, 0),
        },
    )
}

pub fn star2(input: &str) -> String {
    run(
        input,
        true,
        State {
            pos: Vector2::zeros(),
            way: Vector2::new(10, 1),
        },
    )
}
