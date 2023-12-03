use crate::vector::VectorExt;
use lazy_static::lazy_static;
use nalgebra::Vector2;
use regex::Regex;

/// Returns the smallest rectangle which bounds a set of points
fn bounding_rect<'a, I: Iterator<Item = &'a Vector2<i32>>>(
    mut points: I,
) -> Option<(Vector2<i32>, Vector2<i32>)> {
    if let Some(&initial) = points.next() {
        let (mut p1, mut p2) = (initial, initial);

        for point in points {
            if point.x < p1.x {
                p1.x = point.x;
            } else if point.x > p2.x {
                p2.x = point.x;
            }

            if point.y < p1.y {
                p1.y = point.y;
            } else if point.y > p2.y {
                p2.y = point.y;
            }
        }

        Some((p1, p2))
    } else {
        None
    }
}

/// Returns the size of the smallest rectangle which bounds a set of points
fn bounding_rect_size<'a, I: Iterator<Item = &'a Vector2<i32>>>(points: I) -> Option<i32> {
    bounding_rect(points).map(|(p1, p2)| (p2 - p1).taxicab_norm())
}

/// Finds the smallest bounding rectangle of a set of points by scanning through time
///  Returns the number of iterations taken.
///  The points vector contains the best points
fn bounding_rect_smallest(points: &mut Vec<Vector2<i32>>, velocities: &[Vector2<i32>]) -> usize {
    assert_eq!(points.len(), velocities.len());

    let mut prev_score = i32::max_value();
    let mut score = bounding_rect_size(points.iter()).unwrap();
    let mut iterations = 0;

    // Keep adding velocities until we get a higher score than before
    while score < prev_score {
        for (point, velocity) in points.iter_mut().zip(velocities.iter()) {
            *point += *velocity;
        }

        prev_score = score;
        score = bounding_rect_size(points.iter()).unwrap();
        iterations += 1;
    }

    // Go back one iteration to get the best points
    for (point, velocity) in points.iter_mut().zip(velocities.iter()) {
        *point -= *velocity;
    }

    iterations - 1
}

/// Renders a set of points to a string
fn points_to_string(mut points: Vec<Vector2<i32>>) -> String {
    let mut result = String::new();

    if !points.is_empty() {
        // Sort input points into the order to be displayed
        points.sort_unstable_by(|&a, &b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
        points.dedup();

        // Render each point after inserting needed passing
        let (mut cursor, _) = bounding_rect(points.iter()).unwrap();
        for point in points {
            for _ in cursor.y..point.y {
                result.push('\n')
            }
            for _ in cursor.x..point.x {
                result.push(' ')
            }
            result.push('#');
            cursor = point + Vector2::x();
        }
    }

    result
}

/// Parses the input into point and velocity vectors
fn parse_input(input: &str) -> (Vec<Vector2<i32>>, Vec<Vector2<i32>>) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^position=<\s*([0-9-]+)\s*,\s*([0-9-]+)\s*>\s*velocity=<\s*([0-9-]+)\s*,\s*([0-9-]+)\s*>$")
            .unwrap();
    }

    let mut points: Vec<Vector2<i32>> = Vec::new();
    let mut velocities: Vec<Vector2<i32>> = Vec::new();

    for line in input.lines() {
        let parts: Vec<i32> = RE
            .captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str().parse().unwrap())
            .collect();

        points.push(Vector2::from_row_slice(&parts[0..2]));
        velocities.push(Vector2::from_row_slice(&parts[2..4]));
    }

    (points, velocities)
}

pub fn star1(input: &str) -> String {
    let (mut points, velocities) = parse_input(input);
    bounding_rect_smallest(&mut points, &velocities);
    points_to_string(points)
}

pub fn star2(input: &str) -> String {
    let (mut points, velocities) = parse_input(input);
    bounding_rect_smallest(&mut points, &velocities).to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, OUT1);
    star_test!(me1, star1, ME1, ME2);

    star_test!(example1b, star2, IN1, "3");
    star_test!(me2, star2, ME1, "10391");

    const IN1: &str = indoc! {"
        position=< 9,  1> velocity=< 0,  2>
        position=< 7,  0> velocity=<-1,  0>
        position=< 3, -2> velocity=<-1,  1>
        position=< 6, 10> velocity=<-2, -1>
        position=< 2, -4> velocity=< 2,  2>
        position=<-6, 10> velocity=< 2, -2>
        position=< 1,  8> velocity=< 1, -1>
        position=< 1,  7> velocity=< 1,  0>
        position=<-3, 11> velocity=< 1, -2>
        position=< 7,  6> velocity=<-1, -1>
        position=<-2,  3> velocity=< 1,  0>
        position=<-4,  3> velocity=< 2,  0>
        position=<10, -3> velocity=<-1,  1>
        position=< 5, 11> velocity=< 1, -2>
        position=< 4,  7> velocity=< 0, -1>
        position=< 8, -2> velocity=< 0,  1>
        position=<15,  0> velocity=<-2,  0>
        position=< 1,  6> velocity=< 1,  0>
        position=< 8,  9> velocity=< 0, -1>
        position=< 3,  3> velocity=<-1,  1>
        position=< 0,  5> velocity=< 0, -1>
        position=<-2,  2> velocity=< 2,  0>
        position=< 5, -2> velocity=< 1,  2>
        position=< 1,  4> velocity=< 2,  1>
        position=<-2,  7> velocity=< 2, -2>
        position=< 3,  6> velocity=<-1, -1>
        position=< 5,  0> velocity=< 1,  0>
        position=<-6,  0> velocity=< 2,  0>
        position=< 5,  9> velocity=< 1, -2>
        position=<14,  7> velocity=<-2,  0>
        position=<-3,  6> velocity=< 2, -1>
    "};

    const ME1: &str = indoc! {"
        position=< 31351, -51811> velocity=<-3,  5>
        position=< 21001,  31317> velocity=<-2, -3>
        position=<-41347, -41423> velocity=< 4,  4>
        position=<-20557,  52103> velocity=< 2, -5>
        position=<-30975,  41713> velocity=< 3, -4>
        position=< 10618, -20633> velocity=<-1,  2>
        position=< 31358, -10242> velocity=<-3,  1>
        position=< 10564, -10247> velocity=<-1,  1>
        position=<-20586, -31033> velocity=< 2,  3>
        position=< 10617,  52095> velocity=<-1, -5>
        position=<-30972, -51814> velocity=< 3,  5>
        position=< 21000, -10244> velocity=<-2,  1>
        position=<-30968,  20931> velocity=< 3, -2>
        position=<-20573,  20926> velocity=< 2, -2>
        position=<-10222, -41421> velocity=< 1,  4>
        position=< 10561,  10531> velocity=<-1, -1>
        position=<-10202, -31033> velocity=< 1,  3>
        position=<-31004, -20633> velocity=< 3,  2>
        position=<-51734,  10533> velocity=< 5, -1>
        position=<-10198,  10540> velocity=< 1, -1>
        position=< 52168,  20930> velocity=<-5, -2>
        position=<-31001, -31024> velocity=< 3,  3>
        position=<-10201,  52095> velocity=< 1, -5>
        position=< 52124,  20926> velocity=<-5, -2>
        position=<-20573, -20641> velocity=< 2,  2>
        position=<-10190,  20927> velocity=< 1, -2>
        position=< 52129, -31026> velocity=<-5,  3>
        position=< 10568,  31315> velocity=<-1, -3>
        position=< 52126,  41704> velocity=<-5, -4>
        position=<-20610,  52095> velocity=< 2, -5>
        position=< 10568,  10540> velocity=<-1, -1>
        position=<-30988, -20639> velocity=< 3,  2>
        position=< 31347,  10537> velocity=<-3, -1>
        position=< 10579,  20926> velocity=<-1, -2>
        position=<-20605,  31322> velocity=< 2, -3>
        position=< 31382,  31319> velocity=<-3, -3>
        position=< 20979, -20633> velocity=<-2,  2>
        position=<-30976,  20931> velocity=< 3, -2>
        position=< 20996,  10536> velocity=<-2, -1>
        position=< 10604,  10538> velocity=<-1, -1>
        position=<-20553, -31024> velocity=< 2,  3>
        position=<-41338, -10242> velocity=< 4,  1>
        position=< 41765,  41708> velocity=<-4, -4>
        position=< 21007, -31029> velocity=<-2,  3>
        position=< 41758, -20642> velocity=<-4,  2>
        position=<-51743, -41419> velocity=< 5,  4>
        position=<-10171, -41420> velocity=< 1,  4>
        position=< 31366, -51808> velocity=<-3,  5>
        position=<-51741, -20635> velocity=< 5,  2>
        position=< 10569,  41708> velocity=<-1, -4>
        position=< 41741,  52104> velocity=<-4, -5>
        position=< 52144, -20642> velocity=<-5,  2>
        position=< 41762,  10540> velocity=<-4, -1>
        position=< 41733, -31030> velocity=<-4,  3>
        position=< 31369,  10540> velocity=<-3, -1>
        position=<-10196, -41415> velocity=< 1,  4>
        position=< 31391, -20635> velocity=<-3,  2>
        position=< 21002,  31317> velocity=<-2, -3>
        position=<-51770, -31026> velocity=< 5,  3>
        position=<-20587,  10531> velocity=< 2, -1>
        position=< 31395, -10250> velocity=<-3,  1>
        position=< 41786,  41712> velocity=<-4, -4>
        position=< 41789,  20931> velocity=<-4, -2>
        position=< 20967,  52100> velocity=<-2, -5>
        position=< 10605,  31320> velocity=<-1, -3>
        position=< 10609, -20640> velocity=<-1,  2>
        position=< 52153, -20641> velocity=<-5,  2>
        position=<-31004,  20928> velocity=< 3, -2>
        position=< 41778,  31320> velocity=<-4, -3>
        position=<-41363,  52099> velocity=< 4, -5>
        position=< 41737,  31317> velocity=<-4, -3>
        position=< 41758,  52101> velocity=<-4, -5>
        position=<-51786, -51811> velocity=< 5,  5>
        position=< 21002, -31029> velocity=<-2,  3>
        position=< 41765, -31027> velocity=<-4,  3>
        position=<-20565,  52103> velocity=< 2, -5>
        position=< 21007, -10245> velocity=<-2,  1>
        position=< 52124, -10243> velocity=<-5,  1>
        position=< 41753, -41420> velocity=<-4,  4>
        position=<-51754,  20924> velocity=< 5, -2>
        position=<-30964, -51812> velocity=< 3,  5>
        position=< 41775, -20639> velocity=<-4,  2>
        position=< 31395,  41713> velocity=<-3, -4>
        position=< 21007, -10244> velocity=<-2,  1>
        position=< 10592,  20927> velocity=<-1, -2>
        position=<-41377, -10247> velocity=< 4,  1>
        position=<-20557,  10532> velocity=< 2, -1>
        position=< 41733,  20928> velocity=<-4, -2>
        position=<-51781, -10243> velocity=< 5,  1>
        position=< 10593,  41713> velocity=<-1, -4>
        position=<-41379, -10242> velocity=< 4,  1>
        position=<-51783,  31313> velocity=< 5, -3>
        position=<-41337, -41420> velocity=< 4,  4>
        position=<-41338, -20638> velocity=< 4,  2>
        position=< 41749,  52100> velocity=<-4, -5>
        position=< 52142,  52095> velocity=<-5, -5>
        position=< 10612,  10533> velocity=<-1, -1>
        position=< 31391, -51812> velocity=<-3,  5>
        position=< 52125, -41424> velocity=<-5,  4>
        position=< 10612, -41418> velocity=<-1,  4>
        position=< 31342,  20929> velocity=<-3, -2>
        position=<-51770, -20638> velocity=< 5,  2>
        position=< 41750, -10247> velocity=<-4,  1>
        position=< 20951, -10247> velocity=<-2,  1>
        position=<-20557, -10244> velocity=< 2,  1>
        position=<-41363,  20929> velocity=< 4, -2>
        position=< 41782, -20636> velocity=<-4,  2>
        position=< 41746,  52095> velocity=<-4, -5>
        position=< 10576,  52103> velocity=<-1, -5>
        position=<-10198, -31026> velocity=< 1,  3>
        position=< 31387, -10242> velocity=<-3,  1>
        position=<-51758,  41704> velocity=< 5, -4>
        position=< 52148,  52095> velocity=<-5, -5>
        position=<-31004, -10250> velocity=< 3,  1>
        position=<-41358,  20930> velocity=< 4, -2>
        position=<-41386, -31033> velocity=< 4,  3>
        position=< 31387,  10535> velocity=<-3, -1>
        position=<-30996,  52101> velocity=< 3, -5>
        position=< 10619,  20926> velocity=<-1, -2>
        position=< 41754,  41704> velocity=<-4, -4>
        position=<-41339,  31318> velocity=< 4, -3>
        position=< 41761, -31030> velocity=<-4,  3>
        position=<-20608,  31320> velocity=< 2, -3>
        position=< 10560, -31028> velocity=<-1,  3>
        position=<-30972,  41706> velocity=< 3, -4>
        position=< 21004, -51807> velocity=<-2,  5>
        position=< 52125,  41713> velocity=<-5, -4>
        position=<-41355,  41709> velocity=< 4, -4>
        position=< 52176,  20928> velocity=<-5, -2>
        position=<-10188,  10540> velocity=< 1, -1>
        position=<-20557, -31027> velocity=< 2,  3>
        position=< 52132,  52096> velocity=<-5, -5>
        position=< 52140,  20930> velocity=<-5, -2>
        position=< 10584, -10243> velocity=<-1,  1>
        position=<-30943, -51815> velocity=< 3,  5>
        position=<-30947, -31024> velocity=< 3,  3>
        position=< 31378,  52095> velocity=<-3, -5>
        position=<-20571, -10248> velocity=< 2,  1>
        position=< 20999,  31314> velocity=<-2, -3>
        position=<-30963, -51813> velocity=< 3,  5>
        position=< 31390, -41415> velocity=<-3,  4>
        position=<-20609, -41420> velocity=< 2,  4>
        position=< 10568,  31318> velocity=<-1, -3>
        position=< 31342,  20923> velocity=<-3, -2>
        position=< 31347, -20641> velocity=<-3,  2>
        position=<-10193, -20642> velocity=< 1,  2>
        position=<-41347,  10532> velocity=< 4, -1>
        position=<-41379,  10533> velocity=< 4, -1>
        position=< 20959,  20929> velocity=<-2, -2>
        position=< 52135, -20642> velocity=<-5,  2>
        position=< 41761,  31316> velocity=<-4, -3>
        position=<-20568,  10531> velocity=< 2, -1>
        position=<-10193, -20640> velocity=< 1,  2>
        position=<-41342,  41705> velocity=< 4, -4>
        position=<-20568,  41704> velocity=< 2, -4>
        position=< 41745,  10535> velocity=<-4, -1>
        position=<-20603,  10531> velocity=< 2, -1>
        position=< 52157, -10251> velocity=<-5,  1>
        position=< 52142, -10247> velocity=<-5,  1>
        position=< 10576, -31030> velocity=<-1,  3>
        position=< 41750,  52099> velocity=<-4, -5>
        position=< 31390,  10540> velocity=<-3, -1>
        position=<-51781,  41705> velocity=< 5, -4>
        position=< 10600, -10242> velocity=<-1,  1>
        position=< 41737, -20642> velocity=<-4,  2>
        position=<-41370, -41415> velocity=< 4,  4>
        position=< 20980, -10250> velocity=<-2,  1>
        position=< 21007,  31321> velocity=<-2, -3>
        position=< 10562, -20642> velocity=<-1,  2>
        position=<-30952, -41417> velocity=< 3,  4>
        position=<-31004,  52104> velocity=< 3, -5>
        position=< 52129,  10532> velocity=<-5, -1>
        position=<-51730, -41424> velocity=< 5,  4>
        position=<-20613, -51808> velocity=< 2,  5>
        position=<-10214,  41706> velocity=< 1, -4>
        position=<-41368, -20633> velocity=< 4,  2>
        position=< 31403,  31322> velocity=<-3, -3>
        position=< 52153,  20922> velocity=<-5, -2>
        position=<-51736, -41419> velocity=< 5,  4>
        position=< 10568, -20642> velocity=<-1,  2>
        position=< 41741, -20641> velocity=<-4,  2>
        position=< 41743, -10251> velocity=<-4,  1>
        position=<-30944, -31024> velocity=< 3,  3>
        position=< 20956,  10538> velocity=<-2, -1>
        position=<-41350,  20931> velocity=< 4, -2>
        position=<-30992, -51811> velocity=< 3,  5>
        position=<-31001,  31313> velocity=< 3, -3>
        position=< 20959,  41709> velocity=<-2, -4>
        position=<-10214, -41420> velocity=< 1,  4>
        position=<-51766, -20638> velocity=< 5,  2>
        position=< 52140,  20923> velocity=<-5, -2>
        position=< 31358,  31314> velocity=<-3, -3>
        position=< 20956,  31315> velocity=<-2, -3>
        position=< 41789, -31031> velocity=<-4,  3>
        position=<-20585, -41415> velocity=< 2,  4>
        position=< 31390,  10539> velocity=<-3, -1>
        position=< 31370,  20922> velocity=<-3, -2>
        position=<-30970,  31322> velocity=< 3, -3>
        position=< 41785,  31320> velocity=<-4, -3>
        position=<-10162, -41415> velocity=< 1,  4>
        position=<-20569,  31320> velocity=< 2, -3>
        position=< 52177,  41704> velocity=<-5, -4>
        position=< 21009,  31317> velocity=<-2, -3>
        position=< 31345, -41420> velocity=<-3,  4>
        position=<-41335,  52099> velocity=< 4, -5>
        position=<-51733, -20634> velocity=< 5,  2>
        position=<-41345, -51811> velocity=< 4,  5>
        position=<-41368, -20638> velocity=< 4,  2>
        position=<-10203, -41424> velocity=< 1,  4>
        position=<-20597, -51809> velocity=< 2,  5>
        position=<-51778, -41418> velocity=< 5,  4>
        position=< 31400,  41704> velocity=<-3, -4>
        position=< 41734, -31029> velocity=<-4,  3>
        position=< 52184, -31029> velocity=<-5,  3>
        position=< 20967,  41707> velocity=<-2, -4>
        position=<-51746,  31315> velocity=< 5, -3>
        position=< 10600, -41417> velocity=<-1,  4>
        position=< 41775, -31030> velocity=<-4,  3>
        position=< 21010, -41415> velocity=<-2,  4>
        position=<-10166,  41707> velocity=< 1, -4>
        position=< 52166,  41708> velocity=<-5, -4>
        position=<-41355, -31027> velocity=< 4,  3>
        position=<-20597, -31028> velocity=< 2,  3>
        position=< 31355,  31313> velocity=<-3, -3>
        position=<-30972, -41422> velocity=< 3,  4>
        position=<-51725,  20922> velocity=< 5, -2>
        position=< 31382,  20930> velocity=<-3, -2>
        position=<-30946, -51811> velocity=< 3,  5>
        position=< 52132, -20640> velocity=<-5,  2>
        position=<-10177, -51814> velocity=< 1,  5>
        position=< 10605, -20633> velocity=<-1,  2>
        position=< 31376, -20642> velocity=<-3,  2>
        position=<-20573,  52095> velocity=< 2, -5>
        position=<-41358,  52096> velocity=< 4, -5>
        position=<-10218, -41415> velocity=< 1,  4>
        position=< 52180,  31321> velocity=<-5, -3>
        position=< 10605,  20927> velocity=<-1, -2>
        position=<-20557, -41418> velocity=< 2,  4>
        position=< 52164,  20928> velocity=<-5, -2>
        position=< 10565,  10539> velocity=<-1, -1>
        position=< 31395, -51815> velocity=<-3,  5>
        position=< 20967, -20635> velocity=<-2,  2>
        position=< 20967,  10538> velocity=<-2, -1>
        position=< 10562,  10535> velocity=<-1, -1>
        position=<-20605,  20925> velocity=< 2, -2>
        position=<-30948, -31024> velocity=< 3,  3>
        position=<-10206,  52099> velocity=< 1, -5>
        position=<-10217, -51810> velocity=< 1,  5>
        position=< 10596,  10531> velocity=<-1, -1>
        position=< 20977, -51810> velocity=<-2,  5>
        position=<-10213, -10247> velocity=< 1,  1>
        position=< 31378, -10242> velocity=<-3,  1>
        position=< 10580, -51811> velocity=<-1,  5>
        position=<-30999,  41710> velocity=< 3, -4>
        position=<-41387,  31318> velocity=< 4, -3>
        position=< 31374,  52096> velocity=<-3, -5>
        position=< 52169,  10531> velocity=<-5, -1>
        position=<-30994, -31029> velocity=< 3,  3>
        position=< 41777, -41416> velocity=<-4,  4>
        position=< 21004, -31024> velocity=<-2,  3>
        position=< 41736, -20633> velocity=<-4,  2>
        position=< 10579,  20922> velocity=<-1, -2>
        position=<-41387, -51808> velocity=< 4,  5>
        position=<-20561,  10534> velocity=< 2, -1>
        position=<-51728, -31033> velocity=< 5,  3>
        position=< 31352,  20922> velocity=<-3, -2>
        position=<-30944,  41704> velocity=< 3, -4>
        position=< 52135,  41708> velocity=<-5, -4>
        position=<-30948, -10242> velocity=< 3,  1>
        position=<-20568, -41423> velocity=< 2,  4>
        position=< 41753, -31033> velocity=<-4,  3>
        position=< 31360,  52095> velocity=<-3, -5>
        position=<-41338,  31317> velocity=< 4, -3>
        position=< 20975,  20922> velocity=<-2, -2>
        position=< 20951,  41706> velocity=<-2, -4>
        position=<-51746, -20639> velocity=< 5,  2>
        position=< 10587, -10247> velocity=<-1,  1>
        position=<-51778, -20638> velocity=< 5,  2>
        position=<-51741,  20925> velocity=< 5, -2>
        position=< 10605,  41710> velocity=<-1, -4>
        position=<-51781,  52100> velocity=< 5, -5>
        position=< 10600,  31314> velocity=<-1, -3>
        position=<-51778, -31025> velocity=< 5,  3>
        position=<-51778, -41418> velocity=< 5,  4>
        position=< 10592, -10248> velocity=<-1,  1>
        position=<-10177,  20924> velocity=< 1, -2>
        position=<-10185,  31321> velocity=< 1, -3>
        position=< 20987, -41415> velocity=<-2,  4>
        position=<-10211,  52095> velocity=< 1, -5>
        position=< 20986, -10251> velocity=<-2,  1>
        position=<-10218, -41424> velocity=< 1,  4>
        position=<-10181,  31314> velocity=< 1, -3>
        position=<-51760, -31028> velocity=< 5,  3>
        position=< 41759, -41415> velocity=<-4,  4>
        position=<-30954, -51811> velocity=< 3,  5>
        position=<-31004,  20922> velocity=< 3, -2>
        position=< 52164,  52100> velocity=<-5, -5>
        position=<-30964, -31031> velocity=< 3,  3>
        position=< 31358, -41420> velocity=<-3,  4>
        position=< 52141,  41704> velocity=<-5, -4>
        position=< 10585, -51815> velocity=<-1,  5>
        position=<-20557, -51812> velocity=< 2,  5>
        position=<-10169, -10251> velocity=< 1,  1>
        position=< 10576,  52095> velocity=<-1, -5>
        position=<-10170,  41707> velocity=< 1, -4>
        position=< 31363, -41424> velocity=<-3,  4>
        position=<-41339,  20927> velocity=< 4, -2>
        position=<-30978,  52095> velocity=< 3, -5>
        position=<-51786, -10245> velocity=< 5,  1>
        position=<-41371,  31322> velocity=< 4, -3>
        position=<-30996,  20925> velocity=< 3, -2>
        position=< 31374,  20930> velocity=<-3, -2>
        position=< 52175, -10246> velocity=<-5,  1>
        position=< 31374,  41710> velocity=<-3, -4>
        position=<-10182,  52099> velocity=< 1, -5>
        position=<-10206,  41713> velocity=< 1, -4>
        position=<-20557, -31026> velocity=< 2,  3>
        position=< 31377,  52104> velocity=<-3, -5>
        position=< 10608, -41424> velocity=<-1,  4>
        position=<-30964, -20638> velocity=< 3,  2>
        position=<-51741, -10243> velocity=< 5,  1>
        position=< 52156,  31321> velocity=<-5, -3>
        position=< 31354,  31313> velocity=<-3, -3>
        position=< 20980,  20922> velocity=<-2, -2>
        position=<-20556, -10242> velocity=< 2,  1>
        position=< 52129, -20636> velocity=<-5,  2>
        position=< 20976,  41704> velocity=<-2, -4>
        position=<-10198,  41712> velocity=< 1, -4>
        position=< 31394, -41417> velocity=<-3,  4>
        position=<-20573,  52098> velocity=< 2, -5>
        position=<-41350,  20928> velocity=< 4, -2>
        position=< 52156,  10532> velocity=<-5, -1>
        position=<-31002,  20926> velocity=< 3, -2>
        position=<-30999,  10534> velocity=< 3, -1>
        position=< 52177,  52104> velocity=<-5, -5>
        position=<-30978,  31322> velocity=< 3, -3>
        position=<-30959,  31318> velocity=< 3, -3>
        position=<-51741, -31031> velocity=< 5,  3>
        position=< 31344, -10242> velocity=<-3,  1>
        position=<-41386, -10251> velocity=< 4,  1>
        position=< 41738, -10243> velocity=<-4,  1>
        position=< 41778, -10245> velocity=<-4,  1>
        position=< 41774, -41422> velocity=<-4,  4>
        position=<-10166,  20927> velocity=< 1, -2>
        position=< 21011,  41708> velocity=<-2, -4>
        position=<-30988, -31027> velocity=< 3,  3>
        position=<-41339, -51815> velocity=< 4,  5>
        position=< 31385, -10245> velocity=<-3,  1>
        position=<-41369,  52095> velocity=< 4, -5>
        position=< 52129,  10533> velocity=<-5, -1>
        position=< 21010, -41424> velocity=<-2,  4>
        position=<-30964,  10533> velocity=< 3, -1>
    "};

    const ME2: &str = indoc! {"
        #####   ######  ######  ######   ####   #    #  #    #  ######
        #    #  #       #            #  #    #  ##   #  #    #  #
        #    #  #       #            #  #       ##   #   #  #   #
        #    #  #       #           #   #       # #  #   #  #   #
        #####   #####   #####      #    #       # #  #    ##    #####
        #    #  #       #         #     #       #  # #    ##    #
        #    #  #       #        #      #       #  # #   #  #   #
        #    #  #       #       #       #       #   ##   #  #   #
        #    #  #       #       #       #    #  #   ##  #    #  #
        #####   #       #       ######   ####   #    #  #    #  ######
    "};

    const OUT1: &str = indoc! {"
        #   #  ###
        #   #   #
        #   #   #
        #####   #
        #   #   #
        #   #   #
        #   #   #
        #   #  ###
    "};
}
