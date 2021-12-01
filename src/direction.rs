use crate::vector::Vector2;
use std::ops::{Neg, Sub};

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

    /// Converts this direction into a vector with the given size
    ///  This function interprets north as positive y values.
    pub fn to_vec<T: Copy + Sub<Output = T> + Neg<Output = T>>(self, size: T) -> Vector2<T> {
        #[allow(clippy::eq_op)]
        let zero = size - size;
        match self {
            Direction::North => Vector2 { x: zero, y: size },
            Direction::East => Vector2 { x: size, y: zero },
            Direction::South => Vector2 { x: zero, y: -size },
            Direction::West => Vector2 { x: -size, y: zero },
        }
    }

    /// Converts this direction into a vector with the given size
    ///  This function interprets north as negative y values.
    pub fn to_vec_neg<T: Copy + Sub<Output = T> + Neg<Output = T>>(self, size: T) -> Vector2<T> {
        let vec_pos = self.to_vec(size);
        Vector2 {
            x: vec_pos.x,
            y: -vec_pos.y,
        }
    }
}
