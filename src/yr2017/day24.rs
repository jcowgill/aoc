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
        let parts_result: Result<Vec<usize>, ()> =
            s.split('/').map(|part| part.parse().map_err(|_| ())).collect();
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
    input.lines().map(|line| Some(line.parse().unwrap())).collect()
}

/// Finds the "best" bridge which can be built with the given components and start port
///  Returns (length, strength) of the bridge
///  best_test is a comparison function over (new, old) which returns the ordering of the two
///   bridges (the greater one is used)
fn find_bridge<C>(bridge_cmp: &C, port: usize, components: &mut Vec<Option<Component>>)
    -> (usize, usize) where C: Fn((usize, usize), (usize, usize)) -> Ordering {

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
        0, &mut parse_components(input)).1.to_string()
}

/// Find strength of the longest bridge
pub fn star2(input: &str) -> String {
    find_bridge(
        &|(new_len, new_strength), (old_len, old_strength)|
            new_len.cmp(&old_len).then(new_strength.cmp(&old_strength)),
        0, &mut parse_components(input)).1.to_string()
}
