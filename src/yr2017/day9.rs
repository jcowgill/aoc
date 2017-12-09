use std::ops::Add;
use apply_tuple2;

/// Parses the garbage in the given stream, returning the non-canclled amount
///  The opening < must already be consumed
///  Panics on unexpected eof
fn parse_garbage<I>(stream: &mut I) -> i32 where I: Iterator<Item=char> {
    let mut garbage = 0;

    loop {
        match stream.next().unwrap() {
            '>' => return garbage,
            '!' => { stream.next(); () },
            _   => garbage += 1
        };
    }
}

/// Parses one group from a stream, retuning its (score, non-cancelled garbage)
///  The opening { must already be consumed
///  Panics on unexpected eof
fn parse_group<I>(stream: &mut I, depth: i32) -> (i32, i32) where I: Iterator<Item=char> {
    let mut result = (depth, 0);

    loop {
        match stream.next().unwrap() {
            '}' => return result,
            '{' => result = apply_tuple2(i32::add, result, parse_group(stream, depth + 1)),
            '<' => result.1 += parse_garbage(stream),
            ',' => (),
            c   => panic!("invalid character found in group {}", c)
        };
    }
}

/// Parses a string completely
fn parse_full_group(input: &str) -> (i32, i32) {
    let mut char_iter = input.chars();
    assert_eq!(char_iter.next(), Some('{'));
    parse_group(&mut char_iter, 1)
}

/// Parse input stream, return total score
pub fn star1(input: &str) -> String {
    parse_full_group(input).0.to_string()
}

/// Parse input stream, return total non-cancelled garbage
pub fn star2(input: &str) -> String {
    parse_full_group(input).1.to_string()
}
