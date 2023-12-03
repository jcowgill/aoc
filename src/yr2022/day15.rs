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

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "26");
    star_test!(me1, star1, ME, "6078701");

    star_test!(example1b, star2, IN1, "56000011");
    star_test!(me2, star2, ME, "12567351400528");

    const IN1: &str = indoc! {"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "};

    const ME: &str = indoc! {"
        Sensor at x=2983166, y=2813277: closest beacon is at x=3152133, y=2932891
        Sensor at x=2507490, y=122751: closest beacon is at x=1515109, y=970092
        Sensor at x=3273116, y=2510538: closest beacon is at x=3152133, y=2932891
        Sensor at x=1429671, y=995389: closest beacon is at x=1515109, y=970092
        Sensor at x=2465994, y=2260162: closest beacon is at x=2734551, y=2960647
        Sensor at x=2926899, y=3191882: closest beacon is at x=2734551, y=2960647
        Sensor at x=1022491, y=1021177: closest beacon is at x=1515109, y=970092
        Sensor at x=1353273, y=1130973: closest beacon is at x=1515109, y=970092
        Sensor at x=1565476, y=2081049: closest beacon is at x=1597979, y=2000000
        Sensor at x=1841125, y=1893566: closest beacon is at x=1597979, y=2000000
        Sensor at x=99988, y=71317: closest beacon is at x=86583, y=-1649857
        Sensor at x=3080600, y=3984582: closest beacon is at x=3175561, y=4138060
        Sensor at x=3942770, y=3002123: closest beacon is at x=3724687, y=3294321
        Sensor at x=1572920, y=2031447: closest beacon is at x=1597979, y=2000000
        Sensor at x=218329, y=1882777: closest beacon is at x=1597979, y=2000000
        Sensor at x=1401723, y=1460526: closest beacon is at x=1515109, y=970092
        Sensor at x=2114094, y=985978: closest beacon is at x=1515109, y=970092
        Sensor at x=3358586, y=3171857: closest beacon is at x=3152133, y=2932891
        Sensor at x=1226284, y=3662922: closest beacon is at x=2514367, y=3218259
        Sensor at x=3486366, y=3717867: closest beacon is at x=3724687, y=3294321
        Sensor at x=1271873, y=831354: closest beacon is at x=1515109, y=970092
        Sensor at x=3568311, y=1566400: closest beacon is at x=3152133, y=2932891
        Sensor at x=3831960, y=3146611: closest beacon is at x=3724687, y=3294321
        Sensor at x=2505534, y=3196726: closest beacon is at x=2514367, y=3218259
        Sensor at x=2736967, y=3632098: closest beacon is at x=2514367, y=3218259
        Sensor at x=3963402, y=3944423: closest beacon is at x=3724687, y=3294321
        Sensor at x=1483115, y=2119639: closest beacon is at x=1597979, y=2000000
    "};
}
