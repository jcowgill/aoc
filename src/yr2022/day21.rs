use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Expr {
    Human,
    Literal(i64),
    Op(Operator, Box<Expr>, Box<Expr>),
}

fn make_tree(defines: &HashMap<&str, &str>, name: &str, use_human: bool) -> Expr {
    if use_human && name == "humn" {
        Expr::Human
    } else {
        let parts: Vec<_> = defines[name].split_ascii_whitespace().collect();

        if let Ok(literal) = parts[0].parse() {
            Expr::Literal(literal)
        } else {
            let op = match parts[1] {
                "+" => Operator::Add,
                "-" => Operator::Sub,
                "*" => Operator::Mul,
                "/" => Operator::Div,
                op_str => panic!("invalid operator {}", op_str),
            };

            Expr::Op(
                op,
                Box::new(make_tree(defines, parts[0], use_human)),
                Box::new(make_tree(defines, parts[2], use_human)),
            )
        }
    }
}

fn make_tree_from_input(input: &str, use_human: bool) -> Expr {
    make_tree(
        &input.lines().map(|l| l.split_once(':').unwrap()).collect(),
        "root",
        use_human,
    )
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Human => panic!("tried to eval human"),
        Expr::Literal(l) => *l,
        Expr::Op(Operator::Add, l, r) => eval(l) + eval(r),
        Expr::Op(Operator::Sub, l, r) => eval(l) - eval(r),
        Expr::Op(Operator::Mul, l, r) => eval(l) * eval(r),
        Expr::Op(Operator::Div, l, r) => eval(l) / eval(r),
    }
}

fn contains_human(expr: &Expr) -> bool {
    match expr {
        Expr::Human => true,
        Expr::Literal(_) => false,
        Expr::Op(_, l, r) => contains_human(l) || contains_human(r),
    }
}

fn solve(expr: &Expr, result: i64) -> i64 {
    match expr {
        Expr::Human => result,
        Expr::Literal(_) => panic!("cannot solve literal"),
        Expr::Op(op, l, r) => {
            if contains_human(l) {
                let right = eval(r);
                solve(
                    l,
                    match op {
                        Operator::Add => result - right,
                        Operator::Sub => result + right,
                        Operator::Mul => result / right,
                        Operator::Div => result * right,
                    },
                )
            } else {
                let left = eval(l);
                solve(
                    r,
                    match op {
                        Operator::Add => result - left,
                        Operator::Sub => left - result,
                        Operator::Mul => result / left,
                        Operator::Div => left / result,
                    },
                )
            }
        }
    }
}

pub fn star1(input: &str) -> String {
    eval(&make_tree_from_input(input, false)).to_string()
}

pub fn star2(input: &str) -> String {
    if let Expr::Op(_, l, r) = make_tree_from_input(input, true) {
        if contains_human(&r) {
            solve(&r, eval(&l)).to_string()
        } else {
            solve(&l, eval(&r)).to_string()
        }
    } else {
        panic!("invalid root");
    }
}
