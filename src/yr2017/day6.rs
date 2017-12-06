use std::collections::HashMap;

/// Returns the index of the largest element in the iterator
///  Returns the first index on ties
fn first_max_index<T, Iter>(mut iter: Iter) -> Option<usize> where T: Ord, Iter: Iterator<Item=T> {
    match iter.next() {
        Some(value) => {
            let mut max = (value, 0);
            let mut index = 1;
            for value in iter {
                if value > max.0 {
                    max = (value, index);
                }

                index += 1;
            }

            Some(max.1)
        },
        None => None
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

/// Redistribute memory around banks
///  Find first repeated configuration
///  Returns (total iterations taken, iterations in first cycle)
fn find_repeated_redistribution(input: &str) -> (usize, usize) {
    // Generate initial vector
    let mut last_vector: Vec<i32> =
        input.split_whitespace().map(|value| value.parse().unwrap()).collect();

    // Redistribute and store in a hash map until we get a repeated vector
    let mut map = HashMap::new();
    let mut iterations = 0;
    while !map.contains_key(&last_vector) {
        map.insert(last_vector.clone(), iterations);
        iterations += 1;
        redistribute(&mut last_vector);
    }

    (iterations, iterations - map[&last_vector])
}

/// Redistribute memory around banks, return iterations until repeat
pub fn star1(input: &str) -> String {
    find_repeated_redistribution(input).0.to_string()
}

/// Redistribute memory around banks
///  Return length of cycle which repeat generates
pub fn star2(input: &str) -> String {
    find_repeated_redistribution(input).1.to_string()
}
