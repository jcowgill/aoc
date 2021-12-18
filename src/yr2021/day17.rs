use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn parse_input(input: &str) -> (i32, i32, i32, i32) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"target area: x=(\d+)..(\d+), y=(-\d+)..(-\d+)").unwrap();
    }

    RE.captures(input)
        .unwrap()
        .iter()
        .skip(1)
        .map(|c| c.unwrap().as_str().parse().unwrap())
        .next_tuple()
        .unwrap()
}

pub fn star1(input: &str) -> String {
    let (_, _, area_y1, _) = parse_input(input);
    ((area_y1 + 1) * area_y1 / 2).to_string()
}

pub fn star2(input: &str) -> String {
    let (area_x1, area_x2, area_y1, area_y2) = parse_input(input);
    let mut count = 0;

    // If u > 0, we know the probe will hit y = 0 twice by the
    // equation of motion:
    //  y = t(u - t/2)
    //  0 = u - t/2
    //  t = 2u
    //   (t is clearly an integer)
    //
    // Therefore u is bounded by the y value of the bottom of the
    // target. If u was bigger then we will always overshoot. Same
    // applies for u < 0 but using t = 0 as the starting point.
    for uy in (area_y1..=-area_y1).rev() {
        // For the same reason as above, initial x velocity is bounded by
        // the x value of the right of the target.
        for ux in 0..=area_x2 {
            let mut x = 0;
            let mut y = 0;
            let mut highest_y = i32::MIN;

            for t in 0.. {
                x += (ux - t).max(0);
                y += uy - t;

                if y > highest_y {
                    highest_y = y;
                }

                if area_y1 <= y && y <= area_y2 && area_x1 <= x && x <= area_x2 {
                    count += 1;
                    break;
                }

                if y < area_y1 {
                    break;
                }
            }
        }
    }

    count.to_string()
}
