use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Rock(u32);

const ROCKS: [Rock; 5] = [
    Rock(0x0F << 2),
    Rock(0x020702 << 2),
    Rock(0x040407 << 2),
    Rock(0x01010101 << 2),
    Rock(0x0303 << 2),
];

fn rock_collides(rows: &[u8], row: usize, rock: Rock) -> bool {
    u32::from_le_bytes(rows[row..row + 4].try_into().unwrap()) & rock.0 != 0
}

fn push_left(rows: &[u8], row: usize, rock: Rock) -> Rock {
    let new_rock = Rock(rock.0 >> 1);
    if (rock.0 & 0x01010101) == 0 && !rock_collides(rows, row, new_rock) {
        new_rock
    } else {
        rock
    }
}

fn push_right(rows: &[u8], row: usize, rock: Rock) -> Rock {
    let new_rock = Rock(rock.0 << 1);
    if (rock.0 & 0x40404040) == 0 && !rock_collides(rows, row, new_rock) {
        new_rock
    } else {
        rock
    }
}

fn drop_rock(rows: &mut Vec<u8>, jet: &mut impl Iterator<Item = (usize, bool)>, i: u64) -> usize {
    let top = rows.len() + 3;
    let mut rock = ROCKS[(i % 5) as usize];
    let mut row = top;

    // Expand now to simplify code
    rows.resize(top + 4, 0);

    // Move rock as far as possible
    loop {
        if jet.next().unwrap().1 {
            rock = push_right(rows, row, rock);
        } else {
            rock = push_left(rows, row, rock);
        }

        if row == 0 || rock_collides(rows, row - 1, rock) {
            break;
        }

        row -= 1;
    }

    // Ossify the rock
    for (row, rock_value) in rows[row..].iter_mut().zip(rock.0.to_le_bytes().into_iter()) {
        *row |= rock_value;
    }

    // Trim rows list
    if let Some(j) = rows.iter().rposition(|&r| r != 0) {
        rows.truncate(j + 1);
    }

    println!();
    top - row
}

fn solve(input: &str, count: u64) -> String {
    let mut jet = input
        .trim()
        .chars()
        .map(|c| c == '>')
        .enumerate()
        .cycle()
        .peekable();
    let mut rows = Vec::new();
    let mut states = HashMap::new();
    let mut consecutive_found = 0;

    // Run normal simulation until a cycle occurs
    for i in 0..count {
        let rock_id = (i % 5) as u8;
        let jet_id = jet.peek().unwrap().0;
        let drop = drop_rock(&mut rows, &mut jet, i);

        if let Some((old_i, old_row, old_drop)) =
            states.insert((rock_id, jet_id), (i, rows.len(), drop))
        {
            if drop == old_drop {
                if consecutive_found >= 5 {
                    // Found cycle - chop out middle and simulate the rest
                    let remaining_cycles = (count - i) / (i - old_i);
                    let remaining_rocks = (count - i) % (i - old_i);
                    let skipped_rows = remaining_cycles * ((rows.len() - old_row) as u64);

                    for j in 1..remaining_rocks {
                        drop_rock(&mut rows, &mut jet, i + j);
                    }

                    return (skipped_rows + rows.len() as u64).to_string();
                } else {
                    consecutive_found += 1;
                    continue;
                }
            }
        }

        consecutive_found = 0;
    }

    rows.len().to_string()
}

pub fn star1(input: &str) -> String {
    solve(input, 2022)
}

pub fn star2(input: &str) -> String {
    solve(input, 1000000000000)
}
