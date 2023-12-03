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
            "n" => Ok(HexDirection::North),
            "ne" => Ok(HexDirection::NorthEast),
            "se" => Ok(HexDirection::SouthEast),
            "s" => Ok(HexDirection::South),
            "sw" => Ok(HexDirection::SouthWest),
            "nw" => Ok(HexDirection::NorthWest),
            _ => Err(()),
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
            HexDirection::North => HexPointCubic(0, 1, -1),
            HexDirection::NorthEast => HexPointCubic(1, 0, -1),
            HexDirection::SouthEast => HexPointCubic(1, -1, 0),
            HexDirection::South => HexPointCubic(0, -1, 1),
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
        input
            .split(',')
            .filter(|m| !m.is_empty())
            .map(|m| m.parse().unwrap())
            .fold(HexPointCubic(0, 0, 0), HexPointCubic::hex_move),
    )
    .to_string()
}

/// Traverse hex grid, return maximum ever distance from origin
pub fn star2(input: &str) -> String {
    input
        .split(',')
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

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, "ne,ne,ne", "3");
    star_test!(example2a, star1, "ne,ne,sw,sw", "0");
    star_test!(example3a, star1, "ne,ne,s,s", "2");
    star_test!(example4a, star1, "se,sw,se,sw,sw", "3");
    star_test!(empty1, star1, "", "0");
    star_test!(me1, star1, ME, "696");

    star_test!(example1b, star2, "ne,ne,ne", "3");
    star_test!(example2b, star2, "ne,ne,sw,sw", "2");
    star_test!(example3b, star2, "ne,ne,s,s", "2");
    star_test!(example4b, star2, "se,sw,se,sw,sw", "3");
    star_test!(empty2, star2, "", "0");
    star_test!(me2, star2, ME, "1461");

    const ME: &str = indoc! {"
        s,nw,s,sw,ne,sw,nw,s,s,s,s,s,sw,se,se,nw,se,sw,sw,nw,se,se,se,se,se,se,s,ne,se,ne,nw,ne,nw,nw,ne,sw,ne,ne,ne,ne,ne,ne,s,ne,n,s,sw,s,ne,n,n,ne,ne,n,n,ne,n,n,n,n,se,n,n,n,n,n,n,n,n,n,n,n,n,n,sw,nw,ne,nw,n,n,nw,n,nw,nw,n,se,nw,nw,nw,nw,n,nw,nw,nw,nw,ne,nw,nw,nw,s,sw,nw,nw,sw,se,sw,sw,nw,ne,sw,nw,sw,nw,sw,ne,sw,ne,sw,nw,sw,sw,nw,sw,s,nw,sw,nw,sw,s,sw,sw,nw,sw,sw,sw,sw,nw,s,nw,sw,sw,ne,sw,nw,sw,sw,sw,sw,se,se,sw,n,se,sw,sw,sw,sw,sw,sw,s,s,sw,sw,sw,s,sw,s,nw,sw,s,s,n,sw,s,s,s,sw,sw,sw,nw,se,s,s,sw,sw,sw,n,s,ne,s,s,s,s,s,s,s,s,sw,n,s,s,s,s,sw,nw,s,sw,s,s,sw,sw,s,s,s,n,s,s,s,s,s,sw,s,n,s,s,s,s,ne,se,s,s,s,ne,s,s,nw,s,s,s,s,s,s,se,se,se,s,se,se,s,s,nw,s,s,s,s,se,s,se,s,s,s,se,se,se,se,s,se,s,se,nw,n,se,n,n,se,se,se,s,se,se,ne,ne,se,s,n,se,s,se,nw,se,nw,se,se,s,nw,s,se,se,se,n,se,s,se,se,se,se,se,n,se,se,sw,n,n,s,se,n,se,se,se,se,se,n,ne,ne,se,ne,se,se,se,nw,se,se,se,se,se,s,se,nw,se,ne,se,ne,nw,se,se,se,se,sw,ne,se,se,se,se,se,n,se,se,ne,s,se,se,se,se,ne,ne,ne,ne,se,se,se,ne,s,se,se,s,ne,se,ne,ne,ne,se,se,ne,s,s,se,n,n,se,se,se,s,ne,s,ne,se,ne,ne,se,se,ne,se,ne,s,ne,sw,n,ne,ne,ne,se,se,n,se,se,ne,ne,ne,ne,n,ne,se,se,ne,ne,ne,ne,ne,n,ne,se,se,ne,ne,ne,sw,s,ne,ne,se,se,ne,se,se,ne,sw,s,ne,ne,ne,ne,ne,ne,ne,se,sw,ne,se,ne,se,ne,ne,ne,ne,ne,ne,s,nw,ne,ne,ne,ne,nw,ne,ne,se,ne,ne,nw,ne,n,se,ne,ne,ne,ne,ne,nw,n,ne,ne,sw,ne,ne,ne,sw,n,ne,n,ne,ne,s,ne,s,ne,ne,sw,n,n,ne,nw,ne,ne,ne,ne,sw,ne,n,n,ne,n,ne,ne,nw,n,n,ne,ne,se,ne,ne,n,ne,ne,n,n,sw,ne,n,ne,ne,ne,n,ne,s,n,n,ne,n,se,s,s,se,ne,n,s,ne,ne,s,n,n,ne,ne,n,n,ne,se,ne,ne,n,ne,sw,n,ne,ne,ne,n,n,sw,ne,se,n,s,n,ne,n,n,se,ne,ne,sw,n,n,n,n,n,sw,sw,ne,ne,n,s,ne,se,n,ne,n,ne,n,ne,n,n,sw,n,se,n,n,ne,n,n,ne,n,n,ne,ne,se,n,n,sw,ne,sw,n,se,ne,ne,n,nw,n,ne,ne,ne,nw,n,n,s,nw,n,n,n,n,s,n,se,n,n,ne,n,n,se,n,n,n,n,n,n,n,s,n,se,n,n,n,n,n,n,n,n,se,n,s,s,n,n,n,se,n,n,n,s,n,n,n,se,n,n,ne,n,n,nw,n,n,ne,n,n,sw,n,n,s,nw,n,n,n,s,nw,n,n,se,n,sw,n,n,n,nw,n,n,sw,n,n,n,n,n,nw,ne,n,ne,n,n,n,n,nw,s,nw,n,sw,n,n,n,n,n,n,n,n,nw,nw,n,n,nw,ne,n,s,n,n,n,nw,n,n,n,n,n,n,n,nw,n,nw,n,se,nw,nw,nw,n,n,ne,nw,n,n,n,nw,n,n,n,n,n,n,n,nw,nw,n,ne,nw,n,nw,ne,nw,n,n,nw,n,n,n,n,nw,se,n,sw,n,nw,nw,n,n,nw,nw,ne,s,nw,nw,nw,nw,n,n,sw,sw,nw,n,n,nw,se,nw,nw,n,n,sw,n,n,nw,nw,nw,sw,nw,nw,sw,n,se,n,nw,nw,se,ne,nw,nw,n,nw,nw,se,n,nw,nw,nw,n,n,nw,n,n,nw,nw,nw,nw,se,n,n,nw,nw,n,nw,n,nw,nw,n,nw,n,nw,nw,sw,nw,nw,nw,ne,nw,s,nw,sw,nw,sw,s,nw,nw,nw,n,n,n,nw,nw,nw,nw,n,nw,nw,nw,n,nw,n,nw,nw,sw,ne,n,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,s,nw,nw,nw,nw,s,nw,n,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,n,nw,nw,nw,nw,n,nw,nw,n,nw,nw,n,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,sw,nw,nw,se,nw,sw,nw,nw,nw,nw,se,sw,n,nw,nw,nw,ne,n,nw,nw,nw,sw,nw,nw,nw,nw,nw,nw,nw,nw,sw,nw,nw,nw,nw,nw,sw,nw,se,se,ne,nw,sw,nw,se,nw,nw,nw,nw,nw,nw,sw,n,nw,nw,nw,s,nw,ne,nw,sw,ne,ne,nw,nw,nw,ne,ne,sw,nw,nw,nw,sw,nw,nw,s,sw,sw,nw,n,nw,sw,nw,nw,n,sw,sw,sw,nw,se,sw,nw,s,n,sw,nw,nw,n,n,nw,ne,sw,ne,sw,nw,s,nw,nw,sw,nw,nw,nw,sw,ne,nw,nw,nw,s,nw,sw,nw,nw,ne,nw,nw,nw,sw,nw,nw,sw,sw,nw,nw,n,n,sw,sw,sw,nw,sw,n,se,sw,ne,nw,nw,nw,nw,nw,nw,nw,sw,ne,sw,sw,nw,ne,se,sw,n,nw,sw,nw,sw,nw,nw,sw,sw,nw,nw,se,sw,nw,nw,sw,nw,n,se,n,nw,se,sw,nw,nw,sw,ne,nw,sw,sw,n,nw,sw,nw,nw,sw,n,se,sw,sw,nw,nw,nw,sw,nw,nw,sw,sw,sw,sw,sw,n,nw,sw,sw,sw,sw,sw,sw,s,nw,se,nw,nw,sw,sw,nw,sw,se,sw,nw,sw,sw,nw,sw,ne,sw,nw,sw,sw,nw,sw,sw,n,sw,nw,nw,s,nw,sw,nw,s,nw,sw,nw,sw,n,sw,sw,s,sw,se,sw,sw,s,sw,nw,nw,sw,n,sw,n,nw,se,sw,n,sw,n,nw,n,nw,sw,sw,sw,nw,se,nw,sw,nw,sw,nw,sw,sw,nw,sw,sw,sw,sw,sw,sw,nw,sw,sw,ne,nw,sw,sw,nw,sw,sw,sw,ne,nw,sw,sw,ne,sw,sw,s,sw,sw,sw,s,n,sw,nw,sw,sw,sw,sw,sw,sw,sw,sw,sw,ne,sw,sw,sw,nw,sw,s,sw,sw,nw,ne,ne,sw,sw,n,sw,sw,sw,s,sw,nw,sw,sw,sw,sw,n,nw,sw,sw,sw,sw,sw,sw,sw,sw,nw,sw,sw,sw,sw,sw,sw,sw,sw,sw,ne,sw,sw,n,sw,sw,sw,se,sw,sw,ne,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,se,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,n,sw,sw,ne,sw,nw,sw,ne,sw,s,sw,nw,n,sw,sw,sw,sw,sw,sw,sw,ne,sw,sw,sw,sw,n,sw,s,sw,sw,sw,sw,n,n,sw,s,sw,sw,sw,sw,sw,sw,ne,sw,se,sw,ne,sw,sw,se,s,nw,s,sw,sw,sw,sw,sw,sw,sw,sw,s,n,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,n,s,sw,s,sw,s,nw,sw,sw,sw,sw,sw,sw,sw,s,sw,s,s,sw,sw,ne,n,sw,sw,sw,sw,sw,s,s,sw,sw,sw,sw,se,s,sw,sw,n,sw,sw,sw,s,se,s,sw,se,ne,sw,sw,se,sw,sw,ne,sw,s,sw,sw,nw,sw,sw,se,sw,ne,s,sw,sw,sw,nw,sw,sw,s,sw,sw,sw,sw,sw,sw,s,sw,sw,se,sw,se,sw,se,sw,se,s,s,sw,sw,s,sw,s,s,s,ne,sw,s,sw,ne,sw,sw,sw,sw,se,sw,sw,n,ne,sw,sw,sw,sw,sw,s,s,sw,sw,ne,sw,sw,sw,sw,s,s,nw,s,s,sw,s,se,sw,nw,nw,n,s,sw,sw,sw,sw,nw,sw,sw,se,sw,s,nw,sw,s,sw,s,sw,sw,sw,sw,s,sw,s,s,s,sw,s,s,n,sw,sw,sw,s,se,s,nw,s,s,sw,s,s,s,sw,sw,sw,sw,sw,s,s,s,s,sw,s,sw,sw,s,sw,s,sw,sw,s,ne,se,nw,n,sw,s,sw,sw,sw,ne,s,s,s,s,sw,sw,sw,s,s,s,s,ne,sw,s,s,sw,nw,s,s,s,n,s,sw,sw,s,sw,s,n,sw,nw,s,s,sw,sw,sw,n,sw,s,s,nw,sw,s,nw,s,sw,s,s,sw,sw,s,s,s,s,s,s,s,ne,se,sw,s,sw,sw,s,s,s,s,sw,s,n,s,nw,sw,s,nw,sw,sw,s,n,s,nw,sw,sw,s,ne,s,nw,s,sw,sw,s,sw,s,s,sw,sw,s,s,s,s,sw,s,n,nw,s,s,s,sw,sw,sw,n,s,s,s,s,s,sw,sw,sw,sw,s,sw,se,nw,s,s,s,s,s,s,s,sw,sw,s,se,s,s,s,s,s,sw,ne,s,s,ne,sw,n,s,nw,s,sw,se,s,s,s,se,s,sw,ne,sw,sw,sw,sw,sw,s,s,s,s,s,sw,s,s,sw,sw,s,s,s,s,sw,s,sw,s,s,sw,s,s,s,sw,s,s,sw,s,n,n,ne,s,ne,s,s,s,sw,s,sw,s,s,s,s,nw,s,se,s,s,s,sw,s,s,sw,s,n,s,s,s,n,s,s,s,ne,s,nw,s,se,s,nw,ne,s,s,ne,s,ne,s,s,s,s,sw,sw,s,s,s,s,ne,s,se,nw,s,sw,ne,s,nw,s,s,s,s,se,se,s,sw,sw,s,s,sw,s,s,s,s,s,nw,s,n,s,s,s,s,se,s,nw,s,s,s,sw,s,s,nw,s,s,n,s,s,se,s,n,s,s,se,se,s,n,s,se,s,s,s,s,s,s,s,s,ne,s,se,s,s,s,s,s,nw,s,nw,s,s,sw,se,n,s,s,s,s,se,s,nw,s,s,n,s,s,nw,s,nw,s,se,s,s,n,nw,s,s,se,s,s,s,s,s,se,s,ne,s,s,s,ne,s,s,s,s,s,s,s,s,ne,s,s,s,se,nw,nw,s,s,nw,s,s,s,s,s,se,s,se,s,s,s,s,s,s,s,s,s,se,s,ne,nw,s,s,s,s,s,s,se,s,s,n,ne,sw,s,s,s,s,nw,s,se,ne,s,n,se,se,s,s,n,sw,s,s,s,ne,nw,sw,s,s,se,nw,s,sw,n,s,s,s,s,s,s,se,n,s,se,s,s,s,s,s,s,s,se,s,s,s,s,se,s,s,s,s,s,s,s,s,s,s,s,n,s,s,s,s,s,s,s,s,se,s,sw,se,sw,se,se,s,s,s,s,se,s,s,se,nw,s,se,se,s,s,s,se,s,s,se,s,ne,s,se,s,s,s,s,se,se,s,nw,n,s,s,s,s,s,s,s,s,nw,s,ne,nw,s,s,s,se,se,ne,se,sw,se,s,s,nw,s,s,s,se,se,s,se,s,se,se,s,s,se,se,s,s,s,nw,sw,sw,s,ne,sw,ne,s,s,sw,se,s,ne,s,s,s,s,se,s,se,se,s,se,s,se,s,s,se,nw,s,s,s,s,se,se,s,se,s,sw,se,s,se,se,s,s,se,sw,se,s,se,s,nw,nw,s,s,se,s,s,se,s,sw,s,s,s,s,se,s,se,se,se,sw,s,se,s,s,nw,se,s,se,se,se,s,s,s,s,s,nw,s,sw,n,s,s,se,se,s,sw,s,se,s,s,se,s,se,se,se,se,s,s,se,s,se,se,ne,s,s,se,s,ne,nw,n,s,s,se,se,se,se,se,sw,n,se,se,s,nw,se,se,se,s,s,sw,s,sw,se,se,se,se,s,se,se,sw,se,s,se,se,s,ne,n,s,se,s,s,s,s,nw,se,s,se,s,se,s,s,se,se,se,s,se,s,s,se,se,s,s,se,se,se,se,s,se,ne,sw,se,se,nw,se,s,se,sw,sw,se,s,se,se,s,se,n,se,s,s,se,se,se,s,s,s,se,n,se,sw,nw,se,ne,se,n,se,s,n,ne,s,se,se,se,s,ne,se,se,s,n,se,n,sw,se,se,se,s,nw,se,se,se,n,se,s,n,s,nw,sw,se,s,s,s,s,se,se,se,s,se,se,s,se,se,s,n,se,s,sw,se,se,se,se,s,se,se,n,se,nw,se,se,s,sw,se,se,se,se,se,se,ne,se,s,n,se,se,se,s,se,se,se,se,ne,s,se,se,se,s,se,se,se,se,n,se,se,se,se,se,sw,ne,s,s,se,s,ne,se,se,se,se,se,se,s,se,se,se,se,se,se,s,s,se,se,n,se,se,se,nw,n,se,se,se,se,n,se,nw,nw,se,se,ne,se,se,s,se,nw,se,nw,se,se,nw,se,s,se,se,nw,s,s,se,n,se,se,sw,se,se,se,se,se,se,nw,nw,se,se,s,se,se,s,se,se,se,se,ne,s,se,se,se,s,n,se,ne,se,se,se,se,s,se,se,se,se,se,se,ne,n,se,se,se,se,se,se,se,s,se,se,n,se,s,se,se,se,n,se,se,se,se,se,se,se,se,se,se,se,se,n,se,se,se,nw,se,se,s,se,se,sw,se,se,se,se,se,se,se,s,n,se,se,se,se,se,se,sw,ne,se,se,se,se,se,se,se,se,se,se,se,se,se,se,se,se,se,se,sw,se,ne,se,ne,se,se,se,se,se,se,s,se,ne,n,sw,se,se,se,n,se,se,s,se,se,se,se,se,nw,se,sw,se,sw,se,se,se,se,se,se,ne,n,se,sw,sw,se,sw,ne,se,se,s,se,se,se,se,se,se,s,se,se,ne,nw,se,se,se,sw,se,sw,s,n,n,ne,se,n,se,ne,se,ne,se,ne,se,nw,se,se,se,se,sw,se,se,ne,n,se,se,se,se,se,se,nw,se,se,se,se,se,se,se,se,se,s,n,nw,s,se,nw,se,s,ne,se,se,se,se,se,se,se,se,ne,se,nw,ne,sw,se,se,se,se,se,ne,se,sw,sw,nw,se,nw,ne,ne,ne,se,se,se,se,se,se,n,se,se,nw,se,se,ne,se,se,se,ne,se,ne,se,se,se,se,se,se,se,sw,se,ne,sw,se,se,se,se,se,se,s,sw,se,se,nw,se,n,se,se,n,se,se,se,ne,ne,se,ne,se,se,ne,ne,sw,sw,se,s,se,se,sw,se,se,nw,se,se,nw,se,se,se,se,se,se,ne,nw,se,se,se,ne,ne,se,ne,se,s,se,nw,se,sw,se,se,se,s,sw,se,se,s,se,se,se,nw,n,se,se,se,n,se,ne,se,se,se,se,n,ne,ne,n,se,se,se,se,se,nw,se,ne,se,ne,se,se,se,se,se,nw,se,se,sw,se,se,se,se,ne,se,se,s,s,se,ne,nw,ne,se,se,ne,se,s,s,se,se,se,se,se,se,se,se,se,n,ne,se,se,s,se,nw,se,se,se,se,se,se,se,ne,ne,se,se,ne,s,n,se,se,s,se,ne,ne,ne,se,se,se,s,se,se,ne,nw,se,se,se,se,se,se,ne,se,se,se,n,ne,se,se,se,ne,se,se,ne,se,se,se,s,se,se,ne,sw,se,se,ne,se,se,nw,se,se,se,se,n,se,se,ne,se,ne,se,ne,se,s,sw,n,se,ne,ne,se,se,se,sw,se,se,se,ne,se,se,sw,ne,se,se,n,se,nw,se,ne,ne,se,se,se,se,se,sw,n,ne,se,ne,se,se,se,s,ne,ne,ne,ne,se,se,se,n,se,se,sw,se,se,n,ne,se,ne,se,se,se,ne,ne,se,n,se,se,ne,ne,ne,nw,se,ne,se,se,se,se,se,sw,ne,se,nw,ne,se,se,ne,se,se,ne,ne,se,nw,nw,se,ne,ne,ne,se,ne,ne,se,sw,se,se,se,n,se,ne,ne,se,n,ne,ne,se,ne,ne,s,s,se,ne,ne,se,se,nw,se,se,s,se,ne,se,se,ne,nw,se,se,se,ne,n,se,se,ne,se,ne,se,se,n,se,n,ne,sw,ne,sw,se,se,se,se,ne,se,ne,s,se,se,ne,se,ne,nw,ne,se,se,n,se,se,nw,nw,se,sw,ne,se,se,ne,ne,n,ne,se,se,ne,ne,se,se,ne,s,ne,nw,ne,se,s,se,ne,ne,se,se,nw,ne,ne,ne,n,n,ne,s,se,ne,ne,se,ne,se,ne,se,se,se,s,ne,ne,ne,ne,se,ne,sw,sw,s,ne,nw,s,se,se,se,s,nw,ne,n,ne,n,nw,n,ne,ne,se,se,se,ne,se,ne,s,se,se,se,ne,se,se,ne,se,se,se,ne,s,se,ne,se,ne,se,ne,ne,ne,ne,sw,n,ne,ne,se,ne,ne,ne,ne,se,nw,se,ne,s,ne,se,se,ne,s,ne,ne,ne,ne,se,ne,ne,se,ne,se,se,s,ne,ne,ne,nw,ne,n,se,ne,ne,ne,ne,se,se,ne,sw,ne,ne,se,se,ne,nw,se,ne,sw,ne,ne,ne,ne,se,nw,ne,ne,ne,se,nw,ne,ne,ne,nw,nw,se,ne,ne,se,se,se,ne,n,se,se,nw,ne,ne,se,ne,s,se,sw,sw,ne,ne,ne,ne,ne,se,ne,se,se,se,se,ne,se,ne,ne,ne,ne,ne,se,ne,s,se,ne,ne,ne,se,se,se,ne,se,se,se,ne,se,ne,ne,ne,se,se,ne,se,ne,ne,ne,ne,ne,ne,ne,se,ne,ne,ne,n,se,ne,se,se,ne,ne,ne,s,ne,ne,se,nw,se,se,ne,s,ne,ne,ne,ne,ne,ne,ne,ne,ne,n,nw,se,se,ne,ne,ne,ne,s,ne,ne,ne,se,se,ne,ne,se,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,se,ne,ne,ne,ne,ne,se,n,ne,se,s,n,se,ne,ne,se,n,nw,ne,ne,sw,ne,s,ne,se,ne,s,ne,ne,se,ne,ne,ne,sw,se,ne,ne,ne,n,ne,ne,ne,se,ne,ne,se,ne,ne,s,ne,ne,se,se,ne,ne,ne,ne,se,nw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,nw,ne,ne,n,ne,se,ne,ne,se,ne,ne,nw,ne,ne,s,se,nw,ne,n,n,se,ne,ne,se,ne,ne,ne,ne,se,nw,ne,ne,ne,ne,se,se,ne,ne,ne,se,ne,ne,se,se,ne,n,ne,ne,ne,ne,sw,n,ne,s,se,s,ne,nw,ne,se,se,ne,ne,se,ne,ne,s,se,ne,n,ne,ne,se,ne,n,ne,se,ne,ne,ne,ne,ne,ne,ne,s,ne,ne,ne,ne,ne,se,ne,ne,ne,ne,ne,ne,se,ne,ne,ne,ne,ne,nw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,sw,s,ne,ne,ne,sw,ne,se,ne,ne,ne,ne,ne,s,se,ne,ne,ne,ne,ne,nw,nw,ne,ne,ne,ne,ne,ne,se,ne,se,n,ne,sw,ne,n,ne,ne,ne,s,ne,ne,ne,nw,se,ne,ne,ne,ne,ne,ne,ne,n,nw,ne,ne,ne,ne,ne,nw,ne,ne,ne,sw,ne,ne,ne,n,ne,ne,ne,n,se,ne,ne,ne,ne,ne,ne,ne,ne,nw,ne,ne,ne,ne,ne,ne,ne,ne,sw,ne,ne,sw,se,se,s,ne,ne,ne,ne,ne,s,ne,ne,ne,ne,s,ne,ne,nw,ne,ne,n,ne,ne,ne,ne,ne,ne,n,ne,nw,sw,ne,ne,sw,ne,ne,ne,s,ne,ne,ne,sw,ne,n,ne,nw,ne,ne,nw,ne,ne,sw,ne,ne,ne,ne,n,ne,s,se,nw,ne,n,ne,se,ne,ne,s,ne,ne,ne,ne,se,ne,ne,ne,nw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,se,ne,ne,ne,ne,n,se,ne,ne,se,n,sw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,sw,ne,sw,ne,ne,ne,ne,ne,ne,ne,n,ne,ne,ne,ne,ne,nw,ne,ne,ne,ne,se,ne,nw,ne,s,se,n,ne,ne,ne,ne,ne,n,s,nw,ne,ne,ne,ne,ne,ne,n,se,ne,ne,ne,ne,se,se,ne,ne,ne,ne,ne,nw,ne,ne,ne,ne,nw,ne,ne,ne,ne,s,nw,ne,n,ne,n,se,nw,ne,n,n,s,ne,ne,ne,sw,ne,ne,ne,se,s,ne,ne,ne,ne,ne,s,ne,ne,ne,ne,ne,ne,ne,ne,ne,n,ne,ne,ne,ne,se,s,se,ne,n,ne,ne,ne,n,n,ne,ne,sw,n,se,sw,ne,n,n,ne,ne,n,ne,ne,ne,ne,ne,se,ne,ne,ne,ne,ne,se,n,ne,ne,ne,ne,ne,ne,ne,ne,n,n,ne,ne,ne,n,ne,se,ne,ne,s,ne,ne,ne,ne,ne,nw,n,ne,ne,ne,ne,sw,n,s,ne,ne,s,ne,n,n,s,n,ne,se,nw,ne,ne,ne,ne,ne,ne,nw,s,n,n,n,ne,ne,ne,nw,ne,ne,n,ne,se,ne,ne,ne,n,n,ne,ne,ne,ne,ne,n,ne,ne,nw,n,s,ne,nw,n,ne,ne,ne,s,ne,ne,ne,sw,ne,ne,ne,n,n,n,sw,n,se,nw,nw,ne,nw,n,nw,ne,ne,n,ne,n,ne,n,n,ne,ne,ne,se,ne,ne,ne,ne,n,ne,ne,ne,nw,ne,s,ne,ne,se,nw,ne,ne,ne,n,nw,n,n,n,ne,sw,ne,ne,s,ne,n,nw,n,ne,s,sw,ne,n,ne,ne,sw,se,ne,ne,n,n,ne,n,ne,ne,ne,n,ne,nw,s,sw,ne,ne,ne,ne,n,nw,nw,ne,s,ne,nw,ne,ne,sw,ne,n,nw,ne,n,ne,ne,se,s,sw,ne,n,s,n,ne,n,ne,ne,ne,n,ne,ne,ne,ne,n,ne,se,ne,ne,se,ne,sw,ne,ne,n,ne,n,n,ne,n,n,sw,n,ne,n,n,n,ne,ne,n,ne,ne,ne,ne,ne,n,ne,n,se,n,ne,s,n,ne,ne,n,n,ne,ne,ne,ne,ne,n,n,n,n,ne,sw,se,n,sw,ne,n,ne,ne,ne,ne,ne,n,ne,ne,ne,ne,ne,ne,n,ne,n,nw,n,n,se,n,ne,n,ne,ne,se,ne,s,sw,ne,ne,sw,n,ne,se,ne,n,sw,ne,sw,ne,n,ne,n,ne,ne,ne,nw,s,n,n,ne,nw,ne,ne,nw,nw,ne,se,sw,ne,n,n,ne,n,n,n,ne,n,ne,sw,n,se,se,ne,se,n,n,ne,ne,ne,ne,ne,n,sw,n,s,ne,ne,ne,se,ne,n,n,n,ne,ne,n,ne,n,ne,ne,ne,n,se,n,se,s,ne,ne,s,ne,ne,s,ne,n,ne,ne,n,se,ne,ne,ne,n,n,ne,sw,ne,ne,n,n,ne,n,ne,s,ne,ne,ne,ne,ne,sw,ne,sw,sw,n,n,n,ne,sw,ne,nw,nw,n,ne,n,ne,ne,ne,ne,s,s,s,n,se,ne,n,n,ne,ne,s,s,ne,ne,n,ne,se,ne,ne,n,se,ne,n,sw,ne,ne,ne,n,n,n,n,ne,sw,ne,nw,nw,se,ne,ne,ne,se,s,n,ne,ne,nw,ne,ne,n,se,ne,ne,ne,n,n,n,ne,ne,n,ne,n,n,ne,ne,se,ne,n,s,sw,ne,ne,ne,n,n,n,ne,ne,n,ne,ne,ne,ne,n,ne,s,n,n,ne,ne,n,n,ne,n,se,ne,ne,n,n,n,sw,ne,ne,ne,ne,n,s,n,s,n,n,n,n,ne,n,n,n,n,se,ne,n,nw,n,se,n,n,ne,s,ne,ne,n,n,n,se,n,n,ne,sw,n,n,ne,ne,n,n,ne,n,ne,sw,n,n,n,n,n,n,ne,ne,ne,n,ne,ne,s,n,s,nw,ne,n,ne,ne,ne,n,n,nw,ne,n,n,ne,se,n,n,n,n,n,ne,n,sw,se,ne,n,ne,ne,s,ne,ne,ne,ne,nw,ne,n,nw,n,se,n,ne,n,ne,s,se,se,n,ne,ne,n,n,ne,n,n,n,n,ne,n,n,n,sw,n,n,ne,ne,se,n,ne,s,n,n,se,se,n,s,ne,ne,s,sw,ne,ne,n,ne,n,n,n,n,n,se,sw,ne,n,n,n,nw,ne,ne,n,n,ne,n,ne,n,se,s,n,n,n,ne,sw,se,n,nw,ne,n,n,ne,n,n,n,n,nw,n,n,n,n,n,sw,n,n,ne,ne,n,ne,n,n,nw,n,n,ne,sw,n,s,nw,ne,nw,ne,nw,n,ne,n,ne,nw,n,n,s,ne,s,ne,ne,n,n,n,ne,n,n,ne,n,n,n,n,n,se,ne,n,ne,n,n,n,sw,ne,ne,n,se,n,se,ne,n,n,n,se,nw,n,n,n,ne,s,n,nw,n,ne,n,ne,n,ne,se,nw,n,n,s,n,nw,n,n,ne,ne,se,n,n,n,se,n,s,ne,n,n,ne,ne,n,n,ne,n,sw,n,n,ne,ne,ne,nw,n,ne,n,n,s,ne,ne,ne,n,n,ne,ne,n,n,nw,ne,n,n,n,n,se,n,sw,n,nw,ne,n,n,sw,ne,n,ne,n,ne,n,ne,se,n,s,se,ne,n,ne,n,ne,n,n,n,ne,n,s,n,ne,nw,n,ne,ne,n,n,n,ne,n,n,sw,n,n,s,sw,se,ne,ne,ne,ne,n,ne,ne,ne,n,ne,n,se,s,n,ne,nw,nw,s,s,ne,s,s,ne,n,n,n,n,se,n,ne,ne,n,ne,n,ne,n,s,sw,ne,ne,n,nw,n,n,ne,n,n,sw,n,n,se,n,n,n,n,nw,ne,s,ne,nw,n,n,n,se,n,n,n,ne,ne,n,ne,ne,ne,n,se,n,n,n,n,n,n,n,ne,s,n,n,n,n,n,n,n,n,n,n,n,n,sw,n,n,n,ne,n,n,n,nw,n,n,n,n,n,ne,nw,n,n,n,n,n,n,n,ne,n,n,n,sw,n,n,ne,n,n,se,nw,n,n,nw,ne,n,n,sw,n,n,n,n,n,n,n,sw,n,n,n,n,n,ne,n,ne,nw,ne,n,se,n,ne,n,n,n,n,sw,n,ne,n,n,nw,n,n,n,n,n,ne,n,n,ne,n,se,n,n,ne,n,n,n,n,se,n,se,n,nw,n,n,ne,n,ne,n,n,ne,n,n,n,se,n,nw,n,n,ne,n,n,n,ne,n,n,n,ne,n,ne,n,n,n,n,n,ne,n,n,n,n,n,n,n,n,n,n,n,n,n,n,n,n,n,n,n,n,n,ne,sw,n,n,n,n,n,n,s,s,n,ne,n,s,n,n,ne,sw,ne,n,n,ne,nw,n,n,nw,n,s,n,n,ne,nw,n,ne,nw,n,n,se,ne,n,ne,n,n,n,n,n,n,se,nw,s,n,n,n,nw,nw,n,n,n,ne,n,sw,n,ne,sw,n,n,n,ne,ne,n,s,sw,n,n,s,n,s,n,nw,n,n,s,s,n,n,n,s,n,n,sw,ne,ne,n,n,n,n,n,sw,n,ne,se,n,n,n,n,n,n,ne,n,n,n,ne,se,n,n,n,n,s,n,s,n,nw,s,ne,s,se,n,n,se,n,se,ne,se,s,sw,n,n,sw,n,nw,n,ne,n,n,n,n,n,sw,n,n,n,n,n,n,n,n,n,ne,n,n,n,n,n,n,nw,n,n,s,s,n,n,n,n,n,n,n,n,n,n,n,n,se,se,se,sw,s,sw,sw,s,sw,sw,nw,n,sw,se,n,sw,sw,nw,nw,nw,nw,nw,se,nw,nw,nw,sw,nw,nw,nw,n,ne,n,nw,n,n,ne,n,n,n,n,ne,nw,n,n,s,ne,n,sw,ne,n,sw,s,n,ne,n,ne,n,n,ne,n,ne,ne,ne,se,ne,ne,nw,ne,nw,ne,ne,ne,ne,ne,nw,ne,n,ne,sw,se,se,ne,se,ne,se,se,ne,ne,ne,se,ne,se,se,se,sw,se,se,nw,se,se,se,se,nw,se,se,se,se,se,ne,se,se,se,se,nw,s,se,se,se,se,se,nw,s,s,se,se,s,s,s,s,s,s,se,se,s,s,ne,n,s,ne,s,s,nw,ne,s,s,s,s,s,s,s,s,s,s,se,s,s,sw,s,s,nw,n,se,n,s,sw,se,nw,ne,s,s,sw,s,nw,ne,s,sw,sw,sw,n,sw,sw,s,sw,nw,sw,s,s,s,sw,s,ne,sw,nw,n,sw,s,se,sw,sw,sw,sw,sw,s,s,s,s,sw,sw,sw,sw,s,sw,ne,se,nw,sw,sw,sw,ne,n,nw,n,sw,sw,sw,se,sw,sw,sw,sw,sw,sw,sw,sw,ne,sw,se,se,se,sw,nw,sw,sw,sw,sw,nw,nw,sw,nw,sw,sw,sw,sw,sw,sw,nw,nw,nw,se,sw,nw,nw,nw,nw,sw,n,n,nw,nw,nw,nw,nw,sw,sw,sw,sw,sw,s,nw,nw,sw,nw,nw,s,se,sw,s,nw,sw,sw,se,s,sw,sw,nw,nw,n,nw,nw,nw,nw,n,nw,nw,se,nw,sw,nw,nw,nw,nw,nw,n,nw,nw,nw,nw,nw,nw,nw,nw,sw,s,s,nw,nw,sw,nw,se,nw,nw,nw,nw,nw,nw,nw,nw,s,nw,nw,nw,nw,nw,nw,s,nw,s,se,nw,nw,nw,nw,nw,ne,nw,ne,nw,n,nw,nw,nw,nw,nw,nw,nw,nw,ne,s,n,n,nw,nw,n,nw,nw,n,se,nw,n,nw,n,nw,nw,n,n,nw,n,n,n,n,sw,se,se,nw,sw,se,n,nw,n,nw,nw,n,n,nw,n,n,nw,n,nw,nw,nw,n,nw,n,n,nw,se,nw,n,nw,sw,nw,n,sw,n,nw,nw,n,n,n,nw,ne,n,s,n,nw,nw,nw,n,n,n,n,n,n,sw,n,nw,n,se,n,s,nw,n,n,n,n,n,ne,n,ne,n,n,ne,se,n,s,n,n,n,sw,n,s,n,n,n,n,ne,n,n,n,n,n,se,n,sw,ne,ne,s,n,n,s,sw,n,n,n,nw,se,n,n,n,n,se,n,n,n,n,ne,n,se,n,ne,sw,se,n,n,ne,n,n,n,n,n,n,ne,ne,ne,sw,n,n,ne,n,n,sw,n,n,n,ne,n,n,n,nw,n,nw,n,n,n,n,ne,ne,n,n,se,ne,se,n,n,ne,ne,n,n,n,n,n,n,ne,n,n,n,n,se,n,ne,n,ne,sw,s,n,n,n,ne,ne,n,n,se,ne,n,ne,sw,ne,ne,n,ne,n,ne,ne,ne,n,sw,ne,ne,ne,ne,n,ne,n,n,ne,ne,ne,n,nw,n,ne,se,ne,sw,ne,ne,ne,ne,ne,ne,nw,nw,se,n,ne,sw,ne,ne,ne,sw,nw,nw,se,ne,ne,ne,ne,n,nw,ne,ne,ne,n,se,ne,ne,sw,ne,ne,nw,n,nw,ne,nw,n,ne,ne,nw,ne,ne,nw,ne,s,se,ne,ne,ne,ne,ne,ne,ne,sw,s,sw,ne,ne,ne,ne,s,ne,ne,ne,ne,s,ne,ne,s,ne,s,s,ne,ne,n,ne,ne,se,sw,ne,se,ne,ne,ne,ne,ne,sw,ne,se,ne,ne,ne,ne,ne,sw,ne,ne,nw,nw,se,se,ne,ne,ne,se,ne,se,nw,sw,s,ne,ne,ne,n,ne,ne,ne,ne,ne,se,ne,ne,se,ne,se,ne,ne,ne,ne,se,s,sw,se,se,ne,ne,ne,se,se,se,ne,ne,se,nw,se,nw,n,se,ne,se,ne,ne,se,se,se,ne,sw,ne,s,se,ne,se,ne,sw,ne,s,se,ne,sw,sw,ne,ne,se,ne,ne,ne,se,se,se,se,ne,s,ne,ne,ne,se,n,ne,se,ne,s,ne,nw,sw,n,se,ne,sw,sw,ne,ne,nw,sw,se,s,se,se,ne,n,ne,se,se,se,se,ne,se,ne,n,nw,se,se,se,se,se,nw,ne,ne,se,sw,se,se,se,ne,se,ne,se,se,ne,ne,ne,nw,ne,se,ne,se,sw,nw,se,ne,ne,se,se,se,ne,se,se,nw,ne,se,se,se,nw,se,se,se,se,se,nw,se,n,se,se,se,n,n,se,ne,se,se,se,se,ne,n,se,n,se,ne,se,n,se,nw,se,se,se,n,se,nw,se,se,se,se,n,se,se,se,se,ne,se,s,ne,ne,se,se,nw,se,se,se,se,s,se,ne,se,se,se,nw,se,se,se,n,se,se,se,ne,se,se,se,se,se,sw,se,se,se,se,se,se,se,se,se,s,se,se,se,se,se,se,se,se,nw,se,n,se,se,se,se,se,se,s,se,se,se,se,se,s,se,se,se,sw,sw,ne,nw,se,sw,s,se,s,se,se,se,se,s,se,n,se,se,n,ne,s,se,se,s,se,nw,n,se,se,se,se,se,se,sw,se,se,se,se,se,s,se,n,se,nw,sw,se,s,se,se,ne,s,se,se,s,se,se,se,s,s,s,se,se,nw,se,se,sw,se,s,se,se,se,se,se,se,ne,s,sw,sw,se,se,s,se,sw,se,nw,se,s,se,sw,se,s,se,se,se,s,se,se,s,se,s,sw,se,se,nw,s,se,se,se,se,se,se,s,s,s,se,s,se,ne,se,s,s,s,s,n,s,se,s,se,se,se,se,sw,se,s,s,n,s,nw,s,se,s,se,s,s,se,s,se,s,se,se,s,se,se,s,se,se,sw,se,ne,se,s,se,se,s,se,s,nw,nw,se,se,se,s,se,s,s,s,s,s,s,s,s,se,s,s,n,se,se,s,nw,s,nw,se,n,se,se,se,s,s,se,sw,n,se,n,ne,s,se,se,se,s,sw,se,s,se,s,se,se,se,ne,se,se,se,s,s,se,sw,ne,s,se,se,s,se,se,sw,s,s,nw,se,n,nw,s,s,nw,se,s,se,s,s,s,s,s,s,s,sw,se,s,se,s,s,s,s,s,s,s,s,s,ne,s,s,s,se,s,s,se,s,s,s,se,s,s,s,s,s,sw,s,s,s,se,s,se,s,se,s,nw,se,s,se,s,s,se,se,ne,se,se,se,s,se,s,s,ne,s,n,s,s,n,se,se,s,s,nw,s,s,nw,sw,s,s,se,s,s,se,s,s,s,se,s,s,s,se,se,se,s,s,sw,s,s,se,n,s,s,s,s,s,s,se,s,s,ne,s,ne,n,s,s,s,s,s,se,s,s,s,ne,n,s,s,s,s,s,s,s,s,s,s,s,s,se,s,s,s,nw,s,s,n,sw,s,s,nw,s,s,s,s,ne,s,s,n,se,s,s,s,s,nw,nw,s,s,s,s,s,s,s,s,s,n,s,s,s,s,sw,s,s,s,s,se,s,s,sw,n,s,nw,s,s,s,s,s,s,nw,s,sw,n,s,s,s,s,s,s,s,sw,s,s,s,s,s,s,s,s,s,s,ne,s,s,s,s,s,s,s,n,s,se,sw,s,sw,s,sw,s,s,s,s,s,n,se,s,nw,s,s,s,se,s,s,sw,s,nw,ne,sw,sw,sw,s,sw,s,sw,s,sw,s,s,nw,s,s,sw,s,s,s,n,s,s,s,s,sw,sw,s,s,s,nw,se,s,s,s,s,s,s,s,se,s,s,s,s,sw,se,sw,s,s,s,s,s,s,s,s,nw,s,se,s,s,s,s,sw,sw,sw,se,s,sw,n,n,sw,sw,sw,s,sw,s,s,s,s,se,sw,s,sw,s,s,sw,sw,n,s,s,s,sw,s,s,s,s,s,s,s,se,nw,s,sw,sw,s,s,nw,sw,sw,s,sw,sw,nw,s,s,sw,sw,sw,s,sw,ne,s,s,s,s,nw,s,sw,s,s,sw,s,sw,s,s,s,s,s,n,s,sw,sw,s,s,s,s,sw,se,s,se,sw,s,sw,s,sw,s,s,sw,sw,s,s,sw,s,n,sw,s,s,s,s,s,sw,sw,sw,sw,sw,sw,ne,s,sw,s,s,n,ne,se,sw,s,s,sw,sw,n,sw,sw,s,sw,s,sw,s,s,n,n,sw,sw,se,s,s,s,sw,sw,s,ne,sw,sw,sw,s,s,sw,s,s,sw,s,s,ne,sw,s,sw,sw,s,s,sw,ne,sw,s,s,sw,sw,sw,sw,sw,s,sw,sw,sw,sw,s,sw,sw,sw,s,sw,sw,s,s,sw,sw,s,sw,s,s,s,sw,sw,se,s,s,ne,sw,sw,sw,s,sw,ne,sw,sw,s,sw,se,s,nw,sw,sw,sw,sw,se,n,sw,n,se,se,sw,sw,sw,s,sw,sw,sw,s,s,ne,sw,ne,sw,se,sw,sw,sw,sw,ne,sw,sw,s,s,sw,sw,sw,sw,n,nw,sw,sw,sw,n,ne,ne,sw,sw,ne,sw,se,sw,sw,s,s,s,s,sw,s,n,ne,sw,n,sw,sw,s,sw,s,nw,sw,sw,sw,nw,sw,sw,nw,sw,n,s,sw,sw,nw,sw,sw,sw,sw,sw,sw,sw,sw,n,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,n,sw,sw,se,nw,sw,sw,sw,nw,ne,se,sw,n,s,sw,sw,s,sw,nw,sw,sw,se,sw,sw,sw,sw,sw,ne,sw,n,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,s,sw,sw,sw,sw,sw,sw,n,sw,sw,ne,sw,sw,sw,sw,sw,sw,sw,sw,sw,ne,sw,sw,s,ne,sw,sw,sw,s,sw,nw,sw,n,sw,se,sw,s,sw,sw,sw,sw,sw,sw,sw,sw,sw,ne,sw,sw,ne,sw,sw,sw,ne,sw,se,sw,sw,nw,sw,sw,sw,sw,nw,s,sw,sw,sw,n,nw,sw,ne,se,sw,sw,nw,sw,sw,sw,sw,n,sw,n,s,sw,sw,sw,sw,sw,ne,sw,sw,ne,sw,sw,n,sw,sw,n,nw,sw,ne,ne,sw,sw,sw,nw,nw,nw,sw,sw,sw,sw,sw,se,sw,sw,sw,sw,sw,sw,sw,sw,n,sw,sw,sw,sw,ne,sw,sw,nw,sw,sw,sw,sw,sw,nw,se,nw,sw,nw,se,sw,nw,nw,n,sw,sw,sw,sw,s,sw,sw,sw,sw,sw,sw,sw,sw,se,ne,sw,nw,sw,sw,sw,sw,sw,sw,n,n,sw,sw,sw,sw,ne,sw,ne,sw,se,sw,sw,sw,sw,sw,sw,sw,n,sw,ne,nw,nw,sw,se,sw,sw,sw,sw,nw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,nw,sw,sw,n,nw,se,n,sw,sw,s,sw,sw,nw,nw,sw,se,nw,nw,sw,sw,se,ne,sw,sw,ne,nw,sw,sw,nw,sw,nw,nw,nw,se,sw,s,sw,sw,sw,nw,nw,sw,nw,sw,nw,nw,nw,sw,nw,sw,nw,sw,sw,se,nw,sw,sw,n,sw,nw,s,sw,sw,sw,sw,nw,nw,sw,nw,sw,nw,sw,nw,sw,sw,sw,sw,nw,sw,sw,sw,sw,n,sw,sw,sw,se,nw,sw,nw,sw,s,s,nw,sw,s,sw,s,s,nw,n,sw,sw,sw,nw,sw,s,ne,sw,ne,ne,nw,sw,sw,sw,sw,sw,ne,n,sw,sw,n,sw,sw,sw,sw,nw,sw,sw,s,sw,nw,sw,sw,sw,nw,nw,nw,sw,sw,sw,nw,nw,sw,sw,sw,sw,nw,nw,se,s,nw,nw,n,nw,s,se,ne,sw,nw,nw,nw,sw,nw,s,nw,nw,sw,sw,sw,sw,n,sw,nw,nw,sw,nw,sw,sw,sw,sw,sw,nw,sw,sw,nw,sw,nw,nw,se,sw,n,se,se,se,sw,sw,nw,sw,sw,sw,s,sw,sw,nw,ne,nw,sw,sw,sw,nw,nw,sw,nw,sw,sw,sw,ne,sw,sw,nw,sw,nw,nw,sw,nw,ne,nw,nw,nw,sw,sw,sw,sw,s,sw,ne,nw,nw,sw,s,nw,se,sw,sw,sw,sw,sw,nw,nw,nw,sw,sw,nw,nw,se,nw,ne,nw,nw,nw,sw,sw,nw,se,sw,nw,nw,nw,nw,sw,sw,nw,sw,n,nw,sw,nw,sw,nw,nw,sw,sw,nw,se,nw,sw,n,nw,sw,nw,nw,sw,nw,n,nw,nw,nw,n,sw,nw,nw,nw,ne,sw,nw,nw,se,sw,sw,sw,nw,nw,sw,nw,ne,n,nw,nw,s,se,nw,ne,sw,nw,nw,nw,sw,sw,nw,s,sw,sw,nw,nw,nw,sw,nw,nw,nw,sw,nw,sw,sw,n,s,sw,nw,nw,nw,nw,sw,se,nw,sw,nw,nw,sw,n,se,nw,sw,s,ne,n,n,sw,nw,se,nw,nw,ne,nw,nw,nw,sw,nw,sw,se,sw,nw,n,ne,sw,nw,nw,nw,s,nw,nw,nw,nw,nw,nw,sw,sw,se,se,nw,nw,s,nw,nw,se,sw,s,s,nw,sw,s,nw,n,nw,nw,sw,nw,sw,s,nw,nw,nw,sw,nw,nw,nw,nw,sw,nw,n,nw,nw,s,nw,sw,nw,sw,nw,sw,ne,sw,nw,nw,nw,nw,sw,sw,nw,sw,nw,se,nw,nw,sw,sw,s,sw,nw,sw,sw,nw,nw,nw,sw,s,nw,nw,sw,n,nw,nw,sw,nw,sw,nw,nw,nw,ne,nw,sw,nw,n,sw,sw,s,nw,nw,nw,sw,nw,n,sw,ne,nw,nw,nw,nw,nw,sw,se,s,nw,nw,nw,nw,nw,nw,nw,nw,sw,nw,nw,s,ne,sw,nw,nw,nw,nw,nw,ne,nw,nw,nw,se,nw,se,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,n,sw,nw,nw,nw,nw,nw,ne,nw,nw,sw,nw,ne,sw,sw,nw,nw,ne,sw,nw,nw,nw,nw,n,sw,nw,se,nw,nw,nw,nw,se,ne,nw,nw,nw,nw,sw,sw,nw,nw,nw,nw,nw,ne,nw,nw,nw,se,se,nw,s,n,nw,nw,nw,nw,nw,nw,nw,n,nw,nw,nw,nw,nw,sw,nw,nw,sw,nw,nw,sw,nw,nw,nw,n,n,nw,ne,nw,nw,nw,nw,nw,nw,nw,nw,n
    "};
}
