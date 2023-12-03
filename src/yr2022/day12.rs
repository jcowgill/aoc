use std::collections::BTreeSet;

use itertools::Itertools;
use nalgebra::DMatrix;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Node {
    code: u8,
    distance: u32,
}

impl Node {
    fn height(self) -> u8 {
        match self.code {
            b'S' => 0,
            b'E' => 25,
            b'a'..=b'z' => self.code - b'a',
            _ => panic!("invalid code"),
        }
    }
}

fn parse_map(input: &str) -> DMatrix<Node> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().trim().len();
    DMatrix::from_row_iterator(
        rows,
        cols,
        input
            .chars()
            .filter(|&c| c.is_ascii_alphabetic())
            .map(|c| Node {
                code: c as u8,
                distance: u32::MAX,
            }),
    )
}

fn find_node(map: &DMatrix<Node>, code: u8) -> (usize, usize) {
    (0..map.nrows())
        .cartesian_product(0..map.ncols())
        .find(|&p| map[p].code == code)
        .unwrap()
}

fn pathfind(map: &mut DMatrix<Node>, start: (usize, usize), height_fn: impl Fn(u8, u8) -> bool) {
    map[start].distance = 0;

    let mut open = BTreeSet::new();
    open.insert((0, start));

    while let Some((dist, pos)) = open.pop_first() {
        // Probe neighbours
        let node_height = map[pos].height();
        for (oy, ox) in [(0, 1), (2, 1), (1, 0), (1, 2)] {
            let npos = ((pos.0 + oy).wrapping_sub(1), (pos.1 + ox).wrapping_sub(1));
            if let Some(node) = map.get_mut(npos) {
                if node.distance > dist + 1 && height_fn(node_height, node.height()) {
                    // Try this node
                    open.remove(&(node.distance, npos));
                    node.distance = dist + 1;
                    open.insert((node.distance, npos));
                }
            }
        }
    }
}

pub fn star1(input: &str) -> String {
    let mut map = parse_map(input);
    let start = find_node(&map, b'S');
    pathfind(&mut map, start, |c, n| n <= (c + 1));
    map[find_node(&map, b'E')].distance.to_string()
}

pub fn star2(input: &str) -> String {
    let mut map = parse_map(input);
    let start = find_node(&map, b'E');
    pathfind(&mut map, start, |c, n| c == 0 || n >= (c - 1));

    (0..map.nrows())
        .cartesian_product(0..map.ncols())
        .filter(|&p| map[p].height() == 0)
        .map(|p| map[p].distance)
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "31");
    star_test!(me1, star1, ME, "420");

    star_test!(example1b, star2, IN1, "29");
    star_test!(me2, star2, ME, "414");

    const IN1: &str = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "};

    const ME: &str = indoc! {"
        abcccccccccaaaaaaaaaaccccccccccccaaaaaaaaccaaccccccccccccccccccccccccccccccccccccccccccccaaaaaa
        abccccccccccaaaaaaaaaccccccccccccaaaaaaaaaaaacccccccccccaacccacccccccccccccccccccccccccccaaaaaa
        abcccccccccccaaaaaaacccccccccccccaaaaaaaaaaaaaacccccccccaaacaacccccccccaaaccccccccccccccccaaaaa
        abccccccccccaaaaaaccccccccccccccaaaaaaaaaaaaaaaccccccccccaaaaaccccccccccaaacccccccccccccccccaaa
        abccccccccccaaaaaaaccccccccccccaaaaaaaaaaaaaacccccccccccaaaaaacccccccccaaaacccccccccccccccccaac
        abaaccaaccccaaccaaaccccccccaaaaaaaaaaaaaaacaaccccccccccaaaaaaaacccccccccaaalcccccccccccccccaaac
        abaaaaaacccccccccaaccccccccaaaaaacccaaaacccaaccccccccccaaaaaaaaccccccccalllllllcccccccccccccccc
        abaaaaaacccccccaaacccccccccaaaaccccccaaaccccaaaaacccccccccaacccccccaaaakllllllllcccccccaacccccc
        abaaaaaacccccccaaaacccccccccaacccccccaaaccccaaaaacccccccccaacccccccaakkklllpllllccccacaaacccccc
        abaaaaaaaccccccaaaaccccaaccccccccccccccccccaaaaaaccccccccccccccccccckkkkpppppplllcccaaaaaaacccc
        abaaaaaaacaaaccaaaaccaaaaaaccccccccccccccccaaaaaacccccccaaaccccckkkkkkkpppppppplllcddaaaaaacccc
        abcaaaacccaacccccccccaaaaaacccccaaaccccccccaaaaaacccccccaaaaccjkkkkkkkpppppuppplmmdddddaaaccccc
        abccaaaaaaaaaccccccccaaaaaaccccaaaaaacccccccaaacccccccccaaaajjjkkkkkrpppuuuuupppmmmdddddacccccc
        abccccaaaaaaaacccccccaaaaacccccaaaaaacccccccccccccccccccaaacjjjjrrrrrrppuuuuupqqmmmmmddddaccccc
        abccccaaaaaaaaacccccccaaaacccccaaaaaaccccccccccccccccccccccjjjrrrrrrrrpuuuxuvvqqqmmmmmddddccccc
        abccccaaaaaaaaacccccccccccccccccaaaaaccccaacccaccccccccaaccjjjrrrruuuuuuuxxyvvqqqqqmmmmmdddcccc
        abccccaaaaaaaacccccccccaaaccccccaacaaccccaaacaacccaaacaaaccjjjrrrtuuuuuuuxxyvvvqqqqqmmmmdddcccc
        abccaaaaaaaacccccccccccaaaaaccccccccccccccaaaaacccaaaaaaaccjjjrrttttxxxxxxyyvvvvvqqqqmmmmdeeccc
        abccaaaccaaaccccccccaacaaaaacccccccccccccaaaaaacccaaaaaacccjjjrrtttxxxxxxxyyvvvvvvvqqqmmmeeeccc
        abaaaaaaaaaacccaaaccaaaaaaaaaaaccaaaccccaaaaaaaacccaaaaaaaajjjqqrttxxxxxxxyyyyyyvvvqqqnnneeeccc
        SbaaaaaaaaccccaaaaccaaaaaaaaaaaaaaaaacccaaaaaaaaccaaaaaaaaacjjjqqtttxxxxEzzyyyyvvvvqqqnnneeeccc
        abcaaaaaacccccaaaaccccaaaaaaaccaaaaaaccccccaaccccaaaaaaaaaaciiiqqqtttxxxyyyyyyvvvvrrrnnneeecccc
        abcaaaaaacccccaaaacccaaaaaaaaccaaaaaaccccccaaccccaaacaaacccciiiqqqqttxxyyyyyywvvvrrrnnneeeecccc
        abcaaaaaaccccccccccccaaaaaaaaacaaaaacccccccccccccccccaaaccccciiiqqtttxxyyyyyywwrrrrnnnneeeccccc
        abcaaacaacccccaacccccaaaaaaaaacaaaaacccccccccccccccccaaaccccciiiqqttxxxywwyyywwrrrnnnneeecccccc
        abccccccccaaacaaccccccccccacccccccccccccccccccccccccccccccccciiqqqttxxwwwwwwywwrrrnnneeeccccccc
        abccaacccccaaaaaccccccccccccccccccccccccccccccccccccccccaacaaiiqqqttwwwwsswwwwwrrrnnfffeccccccc
        abaaaaccccccaaaaaacccccccccccccccccccccccccccccaaaccccccaaaaaiiqqqttssssssswwwwrrronfffaccccccc
        abaaaaaacccaaaaaaacccccccccccccccccccccccccccaaaaaacccccaaaaaiiqqqssssssssssswrrrooofffaaaacccc
        abaaaaaaccaaaaaacccccccccccccccccccccccccccccaaaaaacccccaaaaaiiqqqppssspppssssrrrooofffaaaacccc
        abaaaaaaccaacaaacccccccccccccccccccccccccccccaaaaaacccccaaaaaiihpppppppppppossrrooofffaaaaacccc
        abaaaaccccccccaacccccccccccccccccccccccccccccaaaaaccccccccaaahhhhppppppppppoooooooofffaaaaccccc
        abaaaaccccccccccaacccccccccccccccccaaacccccccaaaaacccccccccccchhhhhhhhhhggpoooooooffffaaaaccccc
        abccaacccccccacaaaccccccccccccccccaaaaacccccccccccccccccccccccchhhhhhhhhggggoooooffffaacaaacccc
        abccccccccccaaaaacaaccccccccccccccaaaaaccccccccccccccccccccccccchhhhhhhhggggggggggffcaacccccccc
        abccccccccccaaaaaaaaccccccccccccccaaaacccaacccccccccccaccccccccccccccaaaaaggggggggfcccccccccccc
        abccccccccccccaaaaaccccaacccccccccaaaacaaaaccccccccaaaaccccccccccccccaaaacaaagggggcccccccccaccc
        abcccccccccccaaaaacccccaacccccccccaaaaaaaaaccccccccaaaaaaccccccccccccaaaccaaaacccccccccccccaaac
        abcccccccccccaacaaccaaaaaaaacccaaaaaaaaaaaccccccccccaaaaccccccccccccccaccccaaacccccccccccccaaaa
        abccccccccccccccaaccaaaaaaaaccaaaaaaaaaaaccccccccccaaaaacccccccccccccccccccccacccccccccccccaaaa
        abccccccccccccccccccccaaaaacccaaaaaaaaaaaacccccccccaacaacccccccccccccccccccccccccccccccccaaaaaa
    "};
}
