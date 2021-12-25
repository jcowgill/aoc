pub fn star1(input: &str) -> String {
    let width = input.lines().next().unwrap().len();
    let mut grid: Vec<_> = input
        .chars()
        .filter(|c| ['.', '>', 'v'].contains(c))
        .map(|c| c as u8)
        .collect();
    let mut temp_grid = vec![0; grid.len()];

    for steps in 1.. {
        let mut no_moves = true;

        // Move all right cucumbers first
        for y in 0..(grid.len() / width) {
            let mut x = 0;
            while x < width {
                let i = y * width + x;
                let other = if x + 1 < width { i + 1 } else { y * width };

                if grid[i] == b'>' && grid[other] == b'.' {
                    temp_grid[i] = b'.';
                    temp_grid[other] = b'>';
                    no_moves = false;

                    // Advance two grid spaces to avoid moving the same
                    // cucumber again
                    x += 2;
                } else {
                    temp_grid[i] = grid[i];
                    x += 1;
                }
            }
        }

        for x in 0..width {
            let mut y = 0;
            while y < (grid.len() / width) {
                let i = y * width + x;
                let other = if i + width < grid.len() { i + width } else { x };

                if temp_grid[i] == b'v' && temp_grid[other] == b'.' {
                    grid[i] = b'.';
                    grid[other] = b'v';
                    no_moves = false;

                    // Advance two grid spaces to avoid moving the same
                    // cucumber again
                    y += 2;
                } else {
                    grid[i] = temp_grid[i];
                    y += 1;
                }
            }
        }

        if no_moves {
            return steps.to_string();
        }
    }

    unreachable!()
}
