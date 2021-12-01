use crate::vector::Vector3;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Particle {
    position: Vector3<i32>,
    velocity: Vector3<i32>,
    accel: Vector3<i32>,
}

/// Parses a particle description
fn parse_particle(line: &str) -> Particle {
    // We are going to ... err ... cheat and exploit the fact that there is exactly one integer
    // between each comma separated part of the particle expression. Ignore all other non relevant
    // characters to get 9 integers and assign those in order :)
    let ints: Vec<i32> = line
        .split(',')
        .map(|raw_str| {
            raw_str
                .chars()
                .filter(|&c| ('0'..='9').contains(&c) || c == '-')
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .collect();

    assert_eq!(ints.len(), 9);
    Particle {
        position: Vector3 {
            x: ints[0],
            y: ints[1],
            z: ints[2],
        },
        velocity: Vector3 {
            x: ints[3],
            y: ints[4],
            z: ints[5],
        },
        accel: Vector3 {
            x: ints[6],
            y: ints[7],
            z: ints[8],
        },
    }
}

/// Orders particles by closest to origin as time tends to infinity
fn order_range_at_inf(a: &Particle, b: &Particle) -> Ordering {
    a.accel
        .taxicab_norm()
        .cmp(&b.accel.taxicab_norm())
        .then_with(|| {
            // If the accelerations are the same, break ties using the norm of
            // velocity _after_ adding enough acceleration vectors to flip the signs
            // of any elements
            let t = a.velocity.taxicab_norm().max(b.velocity.taxicab_norm());
            (a.velocity + a.accel * t)
                .taxicab_norm()
                .cmp(&(b.velocity + b.accel * t).taxicab_norm())

            // Pray there are no cases (that are important) where the above velocities match :)
        })
}

/// Remove all the duplicates in the given vector using the given equality func
///  Input vector must be sorted
///  Returns true if any duplicates were removed
fn remove_duplicates<T, P>(vec: &mut Vec<T>, mut equal: P) -> bool
where
    P: FnMut(&T, &T) -> bool,
{
    // Remove duplicates by swapping elements
    let old_len = vec.len();
    let mut write_pos = 0;
    let mut skipping = false;

    for read_pos in 0..old_len {
        // Skip over values which are the same as the next value
        if read_pos < old_len - 1 && equal(&vec[read_pos], &vec[read_pos + 1]) {
            skipping = true;
        } else if skipping {
            // Skip this item, but not subsequent items
            skipping = false;
        } else {
            vec.swap(read_pos, write_pos);
            write_pos += 1;
        }
    }

    // Truncate vector to number of items written
    vec.truncate(write_pos);
    old_len != vec.len()
}

/// Find particle which stays closest to origin as time -> infinity
pub fn star1(input: &str) -> String {
    input
        .lines()
        .map(parse_particle)
        .enumerate()
        .min_by(|&(_, ref a), &(_, ref b)| order_range_at_inf(a, b))
        .unwrap()
        .0
        .to_string()
}

/// Find particles left after collisions
pub fn star2(input: &str) -> String {
    let mut particles: Vec<Particle> = input.lines().map(parse_particle).collect();
    let mut last_collision = 0;

    while last_collision < 1000 {
        // Advance all particles by one step
        for p in particles.iter_mut() {
            p.velocity += p.accel;
            p.position += p.velocity;
        }

        // Dedup particles which have collided
        particles.sort_unstable_by_key(|p| p.position);
        if remove_duplicates(&mut particles, |a, b| a.position == b.position) {
            last_collision = 0;
        } else {
            last_collision += 1;
        }
    }

    particles.len().to_string()
}
