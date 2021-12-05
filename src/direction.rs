use std::ops::Neg;

use num::{One, Zero};

use crate::vector::Vector2;

/// A direction in the input grid
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Get direction clockwise of self
    pub fn clockwise(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    /// Get direction anti-clockwise of self
    pub fn anticlockwise(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    /// Get reverse direction
    pub fn reverse(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    /// Converts this direction into a unit vector
    ///  This function interprets north as positive y values.
    pub fn to_vec<T: Zero + One + Neg<Output=T>>(self) -> Vector2<T> {
        match self {
            Direction::North => Vector2 { x: T::zero(), y: T::one() },
            Direction::East => Vector2 { x: T::one(), y: T::zero() },
            Direction::South => Vector2 { x: T::zero(), y: -T::one() },
            Direction::West => Vector2 { x: -T::one(), y: T::zero() },
        }
    }

    /// Converts this direction into a unit vector
    ///  This function interprets north as negative y values.
    pub fn to_vec_neg<T: Zero + One + Neg<Output=T>>(self) -> Vector2<T> {
        let vec_pos = self.to_vec();
        Vector2 {
            x: vec_pos.x,
            y: -vec_pos.y,
        }
    }
}
