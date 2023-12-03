use std::cmp::Ordering;
use std::str::FromStr;

/// A component of the bridge with two ports
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Component(usize, usize);

impl Component {
    /// Tests if a port connects to this component
    ///  Returns the OTHER port size if the port does connect
    fn port_connects(&self, port: usize) -> Option<usize> {
        if port == self.0 {
            Some(self.1)
        } else if port == self.1 {
            Some(self.0)
        } else {
            None
        }
    }

    /// Returns the total strength of this component
    fn strength(&self) -> usize {
        self.0 + self.1
    }
}

impl FromStr for Component {
    type Err = ();
    fn from_str(s: &str) -> Result<Component, ()> {
        let parts_result: Result<Vec<usize>, ()> = s
            .split('/')
            .map(|part| part.parse().map_err(|_| ()))
            .collect();
        let parts = parts_result?;
        if parts.len() == 2 {
            Ok(Component(parts[0], parts[1]))
        } else {
            Err(())
        }
    }
}

/// Parses the list of components into a vector
fn parse_components(input: &str) -> Vec<Option<Component>> {
    input
        .lines()
        .map(|line| Some(line.parse().unwrap()))
        .collect()
}

/// Finds the "best" bridge which can be built with the given components and start port
///  Returns (length, strength) of the bridge
///  best_test is a comparison function over (new, old) which returns the ordering of the two
///   bridges (the greater one is used)
fn find_bridge<C>(
    bridge_cmp: &C,
    port: usize,
    components: &mut Vec<Option<Component>>,
) -> (usize, usize)
where
    C: Fn((usize, usize), (usize, usize)) -> Ordering,
{
    // Try pushing all subcomponents which fit
    let mut best_result = (0, 0);
    for i in 0..components.len() {
        if let Some(c) = components[i] {
            if let Some(other_port) = c.port_connects(port) {
                components[i] = None;
                let sub_result = find_bridge(bridge_cmp, other_port, components);
                components[i] = Some(c);

                let new_result = (sub_result.0 + 1, sub_result.1 + c.strength());
                if bridge_cmp(new_result, best_result) == Ordering::Greater {
                    best_result = new_result;
                }
            }
        }
    }

    best_result
}

/// Find strength of the strongest bridge
pub fn star1(input: &str) -> String {
    find_bridge(
        &|(_, new), (_, old)| new.cmp(&old),
        0,
        &mut parse_components(input),
    )
    .1
    .to_string()
}

/// Find strength of the longest bridge
pub fn star2(input: &str) -> String {
    find_bridge(
        &|(new_len, new_strength), (old_len, old_strength)| {
            new_len.cmp(&old_len).then(new_strength.cmp(&old_strength))
        },
        0,
        &mut parse_components(input),
    )
    .1
    .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "31");
    star_test!(me1, star1, ME, "2006");

    star_test!(example1b, star2, IN1, "19");
    star_test!(me2, star2, ME, "1994");

    const IN1: &str = indoc! {"
        0/2
        2/2
        2/3
        3/4
        3/5
        0/1
        10/1
        9/10
    "};

    const ME: &str = indoc! {"
        24/14
        30/24
        29/44
        47/37
        6/14
        20/37
        14/45
        5/5
        26/44
        2/31
        19/40
        47/11
        0/45
        36/31
        3/32
        30/35
        32/41
        39/30
        46/50
        33/33
        0/39
        44/30
        49/4
        41/50
        50/36
        5/31
        49/41
        20/24
        38/23
        4/30
        40/44
        44/5
        0/43
        38/20
        20/16
        34/38
        5/37
        40/24
        22/17
        17/3
        9/11
        41/35
        42/7
        22/48
        47/45
        6/28
        23/40
        15/15
        29/12
        45/11
        21/31
        27/8
        18/44
        2/17
        46/17
        29/29
        45/50
    "};
}
