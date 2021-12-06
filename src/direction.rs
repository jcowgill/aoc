use nalgebra::{Scalar, Vector2};
use num::{One, Signed, Zero};

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
    pub fn to_vec<T: Scalar + Zero + One + Signed>(self) -> Vector2<T> {
        match self {
            Direction::North => Vector2::y(),
            Direction::East => Vector2::x(),
            Direction::South => -Vector2::y(),
            Direction::West => -Vector2::x(),
        }
    }

    /// Converts this direction into a unit vector
    ///  This function interprets north as negative y values.
    pub fn to_vec_neg<T: Scalar + Zero + One + Signed>(self) -> Vector2<T> {
        match self {
            Direction::North => -Vector2::y(),
            Direction::East => Vector2::x(),
            Direction::South => Vector2::y(),
            Direction::West => -Vector2::x(),
        }
    }
}
