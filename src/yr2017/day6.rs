use std::collections::HashSet;

/// Returns the index of the largest element in the iterator
///  Returns the first index on ties
fn first_max_index<T, Iter>(iter: Iter) -> Option<usize> where T: Ord, Iter: Iterator<Item=T> {
    let mut peekable_iter = iter.peekable();
    if peekable_iter.peek() != None {
        let mut max = (peekable_iter.next().unwrap(), 0);
        let mut index = 1;
        for value in peekable_iter {
            if value > max.0 {
                max = (value, index);
            }

            index += 1;
        }

        Some(max.1)
    } else {
        None
    }
}

/// Performs one redistribution iteration over a vector of banks
fn redistribute(vector: &mut Vec<i32>) {
    // Find largest bucket
    assert!(!vector.is_empty());
    let mut index = first_max_index(vector.iter()).unwrap();

    // Erase bucket and redistribute around the vector
    let mut value = vector[index];
    vector[index] = 0;
    index += 1;
    while value > 0 {
        if index >= vector.len() { index = 0 };
        vector[index] += 1;

        index += 1;
        value -= 1;
    }
}

/// Traverse jump table, incrementing each value after a step
pub fn star1(input: &str) -> String {
    // Generate initial vector
    let mut last_vector: Vec<i32> =
        input.split_whitespace().map(|value| value.parse().unwrap()).collect();

    // Redistribute and store in a hash set until we get a repeated vector
    let mut set = HashSet::new();
    let mut iterations = 0;
    while set.insert(last_vector.clone()) {
        iterations += 1;
        println!("{:?}", last_vector);
        redistribute(&mut last_vector);
    }

    iterations.to_string()
}
