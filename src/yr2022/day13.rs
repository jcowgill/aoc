use std::{cmp::Ordering, fmt::Debug, iter::Peekable, str::FromStr};

use itertools::Itertools;

#[derive(Clone, Debug)]
enum Value {
    Int(u32),
    List(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::Int(_), Value::List(_)) => other.cmp(self).reverse(),
            (Value::List(a), Value::Int(_)) => match a.len() {
                0 => Ordering::Less,
                1 => a[0].cmp(other),
                _ => a[0].cmp(other).then(Ordering::Greater),
            },
            (Value::List(a), Value::List(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Value {}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::Int(v)
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Value::List(v)
    }
}

fn parse_list<I: Iterator<Item = char>>(s: &mut Peekable<I>) -> Vec<Value> {
    let mut result = Vec::new();
    while let Some(v) = parse_value(s) {
        result.push(v);
        if s.next_if_eq(&',').is_none() {
            break;
        }
    }
    result
}

fn parse_value<I: Iterator<Item = char>>(s: &mut Peekable<I>) -> Option<Value> {
    match s.peek() {
        Some('[') => {
            s.next();
            let list = parse_list(s);
            s.next_if_eq(&']').map(|_| list.into())
        }
        Some('0'..='9') => {
            let mut int = 0;
            while let Some(c) = s.next_if(|c| c.is_ascii_digit()) {
                int = int * 10 + (c as u8 - b'0') as u32
            }
            Some(int.into())
        }
        _ => None,
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars().peekable();
        parse_value(&mut iter)
            .filter(|_| iter.next().is_none())
            .ok_or(())
    }
}

pub fn star1(input: &str) -> String {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .enumerate()
        .map(|(i, chunk)| {
            let (a, b) = chunk
                .take(2)
                .map(|s| s.parse::<Value>().unwrap())
                .collect_tuple()
                .unwrap();
            if a < b {
                i + 1
            } else {
                0
            }
        })
        .sum::<usize>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    let mut values: Vec<_> = input
        .lines()
        .filter(|&l| !l.is_empty())
        .map(|l| l.parse::<Value>().unwrap())
        .collect();

    values.push(2.into());
    values.push(6.into());
    values.sort_unstable();

    let pos_2 = 1 + values.iter().position(|v| *v == Value::Int(2)).unwrap();
    let pos_6 = 1 + values.iter().position(|v| *v == Value::Int(6)).unwrap();
    (pos_2 * pos_6).to_string()
}
