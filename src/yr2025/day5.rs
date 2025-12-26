use std::collections::BTreeSet;

fn parse_db(input: &str) -> (Vec<(u64, u64)>, &str) {
    let (db_str, ing_str) = input.split_once("\n\n").unwrap();
    (
        db_str
            .lines()
            .map(|line| {
                let (l, r) = line.split_once('-').unwrap();
                (l.parse().unwrap(), r.parse::<u64>().unwrap() + 1)
            })
            .collect(),
        ing_str,
    )
}

pub fn star1(input: &str) -> String {
    let (db, ingredients_str) = parse_db(input);
    let mut ingredients: BTreeSet<u64> = ingredients_str
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    db.into_iter()
        .map(|(l, r)| ingredients.extract_if(l..r, |_| true).count())
        .sum::<usize>()
        .to_string()
}

fn merge_intervals(db: &mut Vec<(u64, u64)>) {
    db.sort_unstable();

    let mut write_ptr = 0;
    for i in 1..db.len() {
        let (rl, rr) = db[i];
        if rr > rl {
            if rl > db[write_ptr].1 {
                // Non overlapping
                write_ptr += 1;
                db[write_ptr] = (rl, rr);
            } else if rr > db[write_ptr].1 {
                // Overlapping
                db[write_ptr].1 = rr;
            }
        }
    }
    db.truncate(write_ptr + 1);
}

pub fn star2(input: &str) -> String {
    let (mut db, _) = parse_db(input);
    merge_intervals(&mut db);
    db.into_iter().map(|(l, r)| r - l).sum::<u64>().to_string()
}
