use crate::vector::VectorExt;
use nalgebra::Vector2;
use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(Vector2<i32>, Vector2<i32>)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line
                .split_ascii_whitespace()
                .map(|p| p.trim_matches(&['x', 'y', '=', ',', ':'][..]))
                .collect();
            (
                Vector2::new(parts[2].parse().unwrap(), parts[3].parse().unwrap()),
                Vector2::new(parts[8].parse().unwrap(), parts[9].parse().unwrap()),
            )
        })
        .collect()
}

fn large_input(sensors: &[(Vector2<i32>, Vector2<i32>)]) -> bool {
    sensors
        .iter()
        .map(|&(s, _)| s)
        .sum::<Vector2<i32>>()
        .taxicab_norm()
        > 1000
}

fn add_range(ranges: &mut Vec<(i32, i32)>, mut start: i32, mut end: i32) {
    if start <= end {
        ranges.retain(|&(other_start, other_end)| {
            if start.max(other_start) <= end.min(other_end) {
                start = start.min(other_start);
                end = end.max(other_end);
                false
            } else {
                true
            }
        });
        ranges.push((start, end));
    }
}

pub fn star1(input: &str) -> String {
    let sensors = parse_input(input);
    let row = if large_input(&sensors) { 2000000 } else { 10 };

    let mut seen = Vec::new();
    let mut beacons_seen = HashSet::new();

    for (sensor, beacon) in sensors.iter() {
        let range = (sensor - beacon).taxicab_norm();
        let row_dist = (sensor.y - row).abs();
        let row_range = range - row_dist;

        add_range(&mut seen, sensor.x - row_range, sensor.x + row_range);

        if beacon.y == row {
            beacons_seen.insert(beacon.x);
        }
    }

    let total_seen = seen.into_iter().map(|(a, b)| b - a + 1).sum::<i32>();
    (total_seen - beacons_seen.len() as i32).to_string()
}

fn point_seen(sensor_ranges: &[(Vector2<i32>, i32)], point: Vector2<i32>) -> bool {
    sensor_ranges
        .iter()
        .any(|&(sensor, range)| (sensor - point).taxicab_norm() <= range)
}

pub fn star2(input: &str) -> String {
    let sensors = parse_input(input);
    let max_coord = if large_input(&sensors) { 4000000 } else { 20 };
    let sensor_ranges: Vec<_> = sensors
        .into_iter()
        .map(|(sensor, beacon)| (sensor, (sensor - beacon).taxicab_norm()))
        .collect();

    for (sensor, range) in sensor_ranges.iter() {
        // The distress beacon must be somewhere one unit away from
        // the range of this sensor
        for dy in [1, -1] {
            for dx in [1, -1] {
                for i in 1..=(range + 1) {
                    let point = Vector2::new(sensor.x + dx * i, sensor.y + dy * (range + 1 - i));
                    if point.x >= 0
                        && point.y >= 0
                        && point.x <= max_coord
                        && point.y <= max_coord
                        && !point_seen(&sensor_ranges, point)
                    {
                        return (i64::from(point.x) * 4000000 + i64::from(point.y)).to_string();
                    }
                }
            }
        }
    }

    panic!("no point found");
}
