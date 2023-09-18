#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Token {
    Num(u8),
    Add,
    Mul,
    Open,
    Close,
}

fn parse(line: &str) -> impl Iterator<Item = Token> + '_ {
    line.chars()
        .filter(|&c| !c.is_ascii_whitespace())
        .map(|c| match c {
            '0'..='9' => Token::Num(c as u8 - b'0'),
            '+' => Token::Add,
            '*' => Token::Mul,
            '(' => Token::Open,
            ')' => Token::Close,
            _ => panic!("invalid character {c}"),
        })
}

fn pop_ops(values: &mut Vec<u64>, ops: &mut Vec<Token>, pop_mul: bool) {
    loop {
        let f = match ops.last() {
            Some(Token::Add) => |a, b| a + b,
            Some(Token::Mul) if pop_mul => |a, b| a * b,
            _ => return,
        };

        ops.pop();
        let v2 = values.pop().unwrap();
        let v1 = values.last_mut().unwrap();
        *v1 = f(*v1, v2);
    }
}

fn evaluate(line: &str, add_precedence: bool) -> u64 {
    let mut values = Vec::new();
    let mut ops = Vec::new();

    for t in parse(line) {
        match t {
            Token::Num(n) => values.push(n.into()),
            Token::Open => ops.push(t),
            Token::Add => {
                pop_ops(&mut values, &mut ops, !add_precedence);
                ops.push(t);
            }
            Token::Mul => {
                pop_ops(&mut values, &mut ops, true);
                ops.push(t);
            }
            Token::Close => {
                pop_ops(&mut values, &mut ops, true);
                assert_eq!(ops.pop(), Some(Token::Open));
            }
        }
    }

    pop_ops(&mut values, &mut ops, true);
    assert!(ops.is_empty());
    assert_eq!(values.len(), 1);
    values[0]
}

fn run(input: &str, add_precedence: bool) -> String {
    input
        .lines()
        .map(|l| evaluate(l, add_precedence))
        .sum::<u64>()
        .to_string()
}

pub fn star1(input: &str) -> String {
    run(input, false)
}

pub fn star2(input: &str) -> String {
    run(input, true)
}
