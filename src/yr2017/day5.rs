/// Traverse jump table, incrementing each value after a step
///  jt_transform is the jump table transforming function
///   output is the new value given to a table entry given the old
pub fn jump_table_traverse<F>(input: &str, jt_transform: F) -> String where F: Fn(i32) -> i32 {
    let mut jump_table: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut position = 0;
    let mut steps_taken = 0;

    while position >= 0 && position < (jump_table.len() as i32) {
        let new_position = jump_table[position as usize];
        jump_table[position as usize] = jt_transform(new_position);
        position += new_position;
        steps_taken += 1;
    }

    steps_taken.to_string()
}

/// Traverse jump table, incrementing each value after a step
pub fn star1(input: &str) -> String {
    jump_table_traverse(input, |value| value + 1)
}

/// Traverse jump table, with some strange function after each step
pub fn star2(input: &str) -> String {
    jump_table_traverse(input, |value| if value >= 3 { value - 1 } else { value + 1 })
}
