/// Parse the list layer depths into a vector
fn parse_depths(input: &str) -> Vec<(i32, i32)> {
    input.lines().map(|line| {
        let parts: Vec<i32> = line.split(':').map(|s| s.trim().parse().unwrap()).collect();
        assert_eq!(parts.len(), 2);
        (parts[0], parts[1])
    }).collect()
}

/// Calculates the severity of performing a walk
///  time_offset = time to start the walk at
///  depths      = the depths of each layer
/// Returns Some(severity) or None if not caught
fn walk_severity(time_offset: i32, depths: &Vec<(i32, i32)>) -> Option<i32> {
    depths.iter().fold(None, |sum, &(layer, depth)| {
        if (time_offset + layer) % (2 * depth - 2) == 0 {
            Some(sum.unwrap_or(0) + layer * depth)
        } else {
            sum
        }
    })
}

/// Find severity of whole firewall trip
pub fn star1(input: &str) -> String {
    walk_severity(0, &parse_depths(input)).unwrap_or(0).to_string()
}

/// Find smallest time delay which gets through safely
pub fn star2(input: &str) -> String {
    let depths = parse_depths(input);
    let mut time_offset = 0;

    while walk_severity(time_offset, &depths).is_some() {
        time_offset += 1;
    }

    time_offset.to_string()
}
