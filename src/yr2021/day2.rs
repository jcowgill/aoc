use std::str::FromStr;

use strum::EnumString;

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
enum CommandName {
    Forward,
    Down,
    Up,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Command {
    name: CommandName,
    amount: i32,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd_str, amount_str) = s.split_once(' ').ok_or(())?;
        Ok(Command {
            name: cmd_str.parse().map_err(|_| ())?,
            amount: amount_str.parse().map_err(|_| ())?,
        })
    }
}

pub fn star1(input: &str) -> String {
    let mut fwd = 0;
    let mut depth = 0;

    for cmd in input.lines().map(|l| l.parse::<Command>().unwrap()) {
        match cmd.name {
            CommandName::Forward => fwd += cmd.amount,
            CommandName::Down => depth += cmd.amount,
            CommandName::Up => depth -= cmd.amount,
        }
    }

    (fwd * depth).to_string()
}

pub fn star2(input: &str) -> String {
    let mut fwd = 0;
    let mut depth = 0;
    let mut aim = 0;

    for cmd in input.lines().map(|l| l.parse::<Command>().unwrap()) {
        match cmd.name {
            CommandName::Forward => {
                fwd += cmd.amount;
                depth += aim * cmd.amount;
            }
            CommandName::Down => aim += cmd.amount,
            CommandName::Up => aim -= cmd.amount,
        }
    }

    (fwd * depth).to_string()
}
