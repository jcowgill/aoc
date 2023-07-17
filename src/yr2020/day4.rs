use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref FIELDS: [(&'static str, Regex); 7] = [
        ("byr", Regex::new(r"^(19[2-9][0-9]|200[0-2])$").unwrap()),
        (
            "ecl",
            Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap()
        ),
        ("eyr", Regex::new(r"^(202[0-9]|2030)$").unwrap()),
        ("hcl", Regex::new(r"^(#[0-9a-f]{6})$").unwrap()),
        (
            "hgt",
            Regex::new(r"^(1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)$").unwrap()
        ),
        ("iyr", Regex::new(r"^(201[0-9]|2020)$").unwrap()),
        ("pid", Regex::new(r"^([0-9]{9})$").unwrap()),
    ];
}

fn parse_passport(p: &str) -> Vec<(&str, &str)> {
    let mut result: Vec<_> = p
        .split_ascii_whitespace()
        .map(|f| f.split_once(':').unwrap())
        .collect();
    result.retain(|(k, _)| *k != "cid");
    result.sort_unstable();
    result
}

fn has_fields(p: &[(&str, &str)]) -> bool {
    p.iter().map(|(k, _)| k).eq(FIELDS.iter().map(|(k, _)| k))
}

fn validate_field((key, value): (&str, &str)) -> bool {
    FIELDS
        .iter()
        .find(|(k, _)| *k == key)
        .unwrap()
        .1
        .is_match(value)
}

pub fn star1(input: &str) -> String {
    input
        .split("\n\n")
        .filter(|p| has_fields(&parse_passport(p)))
        .count()
        .to_string()
}

pub fn star2(input: &str) -> String {
    input
        .split("\n\n")
        .filter(|p| {
            let fields = parse_passport(p);
            has_fields(&fields) && fields.iter().all(|&f| validate_field(f))
        })
        .count()
        .to_string()
}
