use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Mul};

/// 3D integer vector
///  The ordering is lexographical (ie y and z are only considered if x is equal)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Vector3i {
    x: i32,
    y: i32,
    z: i32
}

impl Vector3i {
    /// Returns the taxicab norm of this vector
    fn taxicab_norm(self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for Vector3i {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vector3i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl AddAssign for Vector3i {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul<i32> for Vector3i {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Vector3i {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<Vector3i> for i32 {
    type Output = Vector3i;
    fn mul(self, rhs: Vector3i) -> Vector3i {
        rhs * self
    }
}

impl fmt::Display for Vector3i {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{},{},{}>", self.x, self.y, self.z)
    }
}

#[derive(Clone, Debug)]
struct Particle {
    position: Vector3i,
    velocity: Vector3i,
    accel: Vector3i
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p={}, v={}, a={}", self.position, self.velocity, self.accel)
    }
}

/// Parses a particle description
fn parse_particle(line: &str) -> Particle {
    // We are going to ... err ... cheat and exploit the fact that there is exactly one integer
    // between each comma separated part of the particle expression. Ignore all other non relevant
    // characters to get 9 integers and assign those in order :)
    let ints: Vec<i32> =
        line.split(',')
            .map(|raw_str|
                 raw_str.chars()
                        .filter(|&c| (c >= '0' && c <= '9') || c == '-')
                        .collect::<String>()
                        .parse()
                        .unwrap())
            .collect();

    assert_eq!(ints.len(), 9);
    Particle {
        position: Vector3i { x: ints[0], y: ints[1], z: ints[2] },
        velocity: Vector3i { x: ints[3], y: ints[4], z: ints[5] },
        accel:    Vector3i { x: ints[6], y: ints[7], z: ints[8] },
    }
}

/// Orders particles by closest to origin as time tends to infinity
fn order_range_at_inf(a: &Particle, b: &Particle) -> Ordering {
    a.accel.taxicab_norm().cmp(&b.accel.taxicab_norm()).then_with(|| {
        // If the accelerations are the same, break ties using the norm of
        // velocity _after_ adding enough acceleration vectors to flip the signs
        // of any elements
        let t = a.velocity.taxicab_norm().max(b.velocity.taxicab_norm());
        (a.velocity + a.accel * t).taxicab_norm().cmp(
            &(b.velocity + b.accel * t).taxicab_norm())

        // Pray there are no cases (that are important) where the above velocities match :)
    })
}

/// Remove all the duplicates in the given vector using the given equality func
///  Input vector must be sorted
///  Returns true if any duplicates were removed
fn remove_duplicates<T, P>(vec: &mut Vec<T>, mut equal: P) -> bool
    where P: FnMut(&T, &T) -> bool {

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
        .min_by(|&(_, ref a), &(_, ref b)| order_range_at_inf(&a, &b))
        .unwrap().0.to_string()
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
