use std::cmp::Ordering;

fn mix(input: &str, reps: usize, key: i64) -> String {
    let data: Vec<_> = input
        .lines()
        .map(|l| key * l.parse::<i64>().unwrap())
        .collect();
    let mut mixed_data: Vec<_> = (0..data.len()).collect();

    for _ in 0..reps {
        for i in 0..data.len() {
            let pos = mixed_data.iter().position(|&j| i == j).unwrap();
            let new_pos = (pos as i64 + data[i]).rem_euclid(data.len() as i64 - 1) as usize;

            match new_pos.cmp(&pos) {
                Ordering::Greater => mixed_data[pos..=new_pos].rotate_left(1),
                Ordering::Less => mixed_data[new_pos..=pos].rotate_right(1),
                Ordering::Equal => (),
            }
        }
    }

    let zero_pos = mixed_data.iter().position(|&i| data[i] == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| data[mixed_data[(i + zero_pos) % mixed_data.len()]])
        .sum::<i64>()
        .to_string()
}

pub fn star1(input: &str) -> String {
    mix(input, 1, 1)
}

pub fn star2(input: &str) -> String {
    mix(input, 10, 811589153)
}
