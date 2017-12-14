use std::ops::BitXor;

/// Result of the knot hasher (16 bytes)
pub type KnotHashResult = [u8; 16];

/// Lengths to append to hash input
const LENGTHS_APPEND: [u8; 5] = [17, 31, 73, 47, 23];

/// Parses the number of elements and lengths
fn parse_lengths(input: &str) -> (usize, Vec<u8>) {
    // Extract optional element count
    let mut semicolon_parts: Vec<&str> = input.splitn(2, ';').collect();
    let elements = if semicolon_parts.len() == 2 {
        semicolon_parts.remove(0).trim().parse().unwrap()
    } else {
        256
    };

    // Extract lengths and return result
    (elements, semicolon_parts[0].split(',').flat_map(|s| s.trim().parse()).collect())
}

/// Reverses part of a cyclic list starting at pos
fn reverse_cyclic_sublist<T>(list: &mut Vec<T>, pos: usize, length: usize) {
    let mut left = pos;
    let mut right = pos + length - 1;
    if right >= list.len() { right -= list.len() }

    assert!(left < list.len());
    assert!(right < list.len());

    // Start at each end of the reverse, swap values, move ends closer
    for _ in 0..(length / 2) {
        list.swap(left, right);
        left = if left == list.len() - 1 { 0 } else { left + 1 };
        right = if right == 0 { list.len() - 1 } else { right - 1 };
    }
}

/// Run a single knot hash round
///  knot = the knot to modify
///  lengths = iterator over the lengths to use
///  initial_pos = initial position to start rotating at
///  initial_skip = initial number of positions to skip after each length
/// Returns final position
fn knot_round<T, I>(mut knot: &mut Vec<T>, lengths: I, initial_pos: usize, initial_skip: usize) -> usize
    where I: Iterator, I::Item: Into<usize> {

    assert!(initial_pos < knot.len());

    let mut pos = initial_pos;
    for (i, length) in lengths.enumerate() {
        let length_usize: usize = length.into();
        reverse_cyclic_sublist(&mut knot, pos, length_usize);
        pos = (pos + length_usize + i + initial_skip) % knot.len();
    }

    pos
}

/// Runs the full knot hash of an iterator of bytes
///  This function is used by other days so don't change the signature
pub fn knot_hash<I>(input: I) -> KnotHashResult where I: Iterator<Item=u8> + Clone {
    let full_input = input.chain(LENGTHS_APPEND.iter().cloned());
    let full_input_len = full_input.clone().count();

    // Rust ranges have exclusive upper bounds, so we have to add 255 manually
    let mut knot: Vec<u8> = (0..255).collect();
    knot.push(255);

    // Run 64 rounds of hashing
    let mut pos = 0;
    for round in 0..64 {
        pos = knot_round(&mut knot, full_input.clone(), pos, full_input_len * round);
    }

    // Generate result by XORing blocks of 16 bytes together
    let mut result: KnotHashResult = [0; 16];
    for dense_byte in 0..16 {
        result[dense_byte] = knot[(16 * dense_byte)..(16 * (dense_byte + 1))]
            .iter().fold(0, u8::bitxor);
    }

    result
}

/// Do some knot twisting, return value of first two numbers multiplied
pub fn star1(input: &str) -> String {
    let (elements, lengths) = parse_lengths(input);
    assert!(elements >= 2);

    let mut knot: Vec<usize> = (0..elements).collect();
    knot_round(&mut knot, lengths.iter().cloned(), 0, 0);
    (knot[0] * knot[1]).to_string()
}

/// Hash given input and return a hex string of the result
pub fn star2(input: &str) -> String {
    knot_hash(input.bytes()).iter()
        .fold(String::new(), |acc, &b| acc + &format!("{:02x}", b))
}
