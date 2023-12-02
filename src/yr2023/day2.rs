fn parse_input(input: &str) -> impl Iterator<Item = (u32, Vec<[u32; 3]>)> + '_ {
    input.lines().map(|line| {
        let (game, sets) = line.split_once(':').unwrap();
        (
            game.strip_prefix("Game ").unwrap().parse().unwrap(),
            sets.split(';')
                .map(|set| {
                    let mut result = [0; 3];
                    for cube in set.split(',') {
                        let (num_str, ty_str) = cube.trim().split_once(' ').unwrap();
                        let ty_idx = match ty_str {
                            "red" => 0,
                            "green" => 1,
                            "blue" => 2,
                            _ => panic!("invalid cube type: {ty_str}"),
                        };
                        result[ty_idx] = num_str.parse().unwrap();
                    }
                    result
                })
                .collect(),
        )
    })
}

pub fn star1(input: &str) -> String {
    parse_input(input)
        .filter(|(_, sets)| {
            sets.iter()
                .all(|set| set[0] <= 12 && set[1] <= 13 && set[2] <= 14)
        })
        .map(|(i, _)| i)
        .sum::<u32>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    parse_input(input)
        .map(|(_, sets)| {
            sets.into_iter()
                .fold([0; 3], |acc, set| {
                    [acc[0].max(set[0]), acc[1].max(set[1]), acc[2].max(set[2])]
                })
                .into_iter()
                .product::<u32>()
        })
        .sum::<u32>()
        .to_string()
}
