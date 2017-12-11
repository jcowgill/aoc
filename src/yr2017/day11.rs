use std::ops::{Add, Neg, Sub};
use std::str::FromStr;

/// The 6 directions you can move on the hex plane
///  Directions have the flat side at the top
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum HexDirection {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl FromStr for HexDirection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "n"  => Ok(HexDirection::North),
            "ne" => Ok(HexDirection::NorthEast),
            "se" => Ok(HexDirection::SouthEast),
            "s"  => Ok(HexDirection::South),
            "sw" => Ok(HexDirection::SouthWest),
            "nw" => Ok(HexDirection::NorthWest),
            _    => Err(())
        }
    }
}

/// Trait for objects which can be "moved" in a hex direction
trait HexMove {
    type Output;

    /// Move self in the given direction
    fn hex_move(self, direction: HexDirection) -> Self::Output;
}

/// A point on a hex map represented in "cubic" form
///  All points must have: x + y + z = 0
///  Hex cells are "flat-topped"
///  x goes west to east
///  y goes south south east to north north west
///  z goes north north east to south south west
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct HexPointCubic(i32, i32, i32);

impl Add<HexPointCubic> for HexPointCubic {
    type Output = HexPointCubic;

    fn add(self, rhs: HexPointCubic) -> HexPointCubic {
        HexPointCubic(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Neg for HexPointCubic {
    type Output = HexPointCubic;

    fn neg(self) -> HexPointCubic {
        HexPointCubic(-self.0, -self.1, -self.2)
    }
}

impl Sub<HexPointCubic> for HexPointCubic {
    type Output = HexPointCubic;

    fn sub(self, rhs: HexPointCubic) -> HexPointCubic {
        self + -rhs
    }
}

impl HexMove for HexPointCubic {
    type Output = HexPointCubic;

    fn hex_move(self, direction: HexDirection) -> HexPointCubic {
        self + match direction {
            HexDirection::North     => HexPointCubic(0, 1, -1),
            HexDirection::NorthEast => HexPointCubic(1, 0, -1),
            HexDirection::SouthEast => HexPointCubic(1, -1, 0),
            HexDirection::South     => HexPointCubic(0, -1, 1),
            HexDirection::SouthWest => HexPointCubic(-1, 0, 1),
            HexDirection::NorthWest => HexPointCubic(-1, 1, 0),
        }
    }
}

/// Returns the taxicab norm of the given vector
///  This is the number of hex cells you must traverse to get back to the origin
fn hex_taxicab_norm(vector: HexPointCubic) -> i32 {
    assert_eq!(vector.0 + vector.1 + vector.2, 0);
    (vector.0.abs() + vector.1.abs() + vector.2.abs()) / 2
}

/// Traverse hex grid, return distance from origin
pub fn star1(input: &str) -> String {
    hex_taxicab_norm(
        input.split(',')
        .filter(|m| !m.is_empty())
        .map(|m| m.parse().unwrap())
        .fold(HexPointCubic(0, 0, 0), HexPointCubic::hex_move)
    ).to_string()
}

/// Traverse hex grid, return maximum ever distance from origin
pub fn star2(input: &str) -> String {
    input.split(',')
        .filter(|m| !m.is_empty())
        .map(|m| m.parse().unwrap())
        .scan(HexPointCubic(0, 0, 0), |pos, m| {
            *pos = pos.hex_move(m);
            Some(hex_taxicab_norm(*pos))
        })
        .max()
        .unwrap_or(0)
        .to_string()
}
