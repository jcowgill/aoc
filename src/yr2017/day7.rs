use std::collections::HashMap;

/// Structure representing an tower and its children
#[derive(Debug)]
struct Tower {
    name: String,
    weight: i32,
    children: Vec<Tower>,
}

/// Type used for towers without their children resolved
#[derive(Debug)]
struct UnresolvedTower {
    name: String,
    weight: i32,
    children: Vec<String>,
}

/// Parses a single unresolved tower line
fn parse_unresolved_tower(line: &str) -> UnresolvedTower {
    // Split up the different parts of the string
    let head_tail: Vec<&str> = line.split("->").collect();
    assert!(head_tail.len() == 1 || head_tail.len() == 2);

    let name_weight: Vec<&str> = head_tail[0].split('(').collect();
    assert_eq!(name_weight.len(), 2);

    let name = name_weight[0].trim();
    let weight = name_weight[1].trim_end().trim_end_matches(')').trim();
    let children: Vec<&str> = match head_tail.get(1) {
        Some(value) => value.split(',').map(|s| s.trim()).collect(),
        None => vec![],
    };

    // Return generated structure
    UnresolvedTower {
        name: String::from(name),
        weight: weight.parse().unwrap(),
        children: children.iter().map(|s| s.to_string()).collect(),
    }
}

/// Parses a list of towers into a hashmap containing unresolved towers
fn parse_tower_list(input: &str) -> HashMap<String, UnresolvedTower> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let tower = parse_unresolved_tower(line);
        map.insert(tower.name.clone(), tower);
    }

    map
}

/// Resolved a map of unresolved towers into a single tower tree
///  Panics if cycles or multiple separate trees are found
fn resolve_towers(mut input: HashMap<String, UnresolvedTower>) -> Tower {
    // A tower is resolved when all its children are now Towers
    //  Only towers without parents are in this map
    let mut resolved: HashMap<String, Tower> = HashMap::new();

    // We repeatedly scan the input map for resolvable towers and move them
    // across into the resolved map. Eventually the input map should be empty
    // and the resolved map will contain one final root tower.
    while !input.is_empty() {
        // Find a resolvable tower
        let resolvable = input
            .iter()
            .find(|&(_, value)| value.children.iter().all(|s| resolved.contains_key(s)))
            .unwrap()
            .0
            .clone();

        // Remove it from the input and convert to a resolved tower
        let value = input.remove(&resolvable).unwrap();
        let new_tower = Tower {
            name: value.name,
            weight: value.weight,
            children: value
                .children
                .iter()
                .map(|s| resolved.remove(s).unwrap())
                .collect(),
        };

        resolved.insert(resolvable, new_tower);
    }

    // Extract last entry in hashmap
    assert!(resolved.len() == 1);
    let result = resolved.drain().next().unwrap().1;
    result
}

/// Checks a tower to ensure all the weights are correct
///  Returns: Ok(tower weight), Err(fixed weight)
fn check_tower(tower: &Tower) -> Result<i32, i32> {
    // Recurse to children and return if an error result is found
    let child_results_err: Result<Vec<i32>, i32> = tower.children.iter().map(check_tower).collect();
    let child_results = child_results_err?;

    // Check that all children are balanced
    if child_results.len() >= 3 {
        // Choose the "correct" weight
        let good_weight = if child_results[0] == child_results[1] {
            child_results[0]
        } else {
            assert!(child_results[2] == child_results[0] || child_results[2] == child_results[1]);
            child_results[2]
        };

        // Find child which doesn't match this weight
        if let Some((i, value)) = child_results
            .iter()
            .enumerate()
            .find(|&(_, &value)| value != good_weight)
        {
            // Return corrected weight
            return Err(tower.children[i].weight + good_weight - value);
        }
    } else if child_results.len() == 2 && child_results[0] != child_results[1] {
        // There are two children with different weights
        //  Solving this is complex, so we pretend it doesn't exist :)
        panic!("tower with 2 children of different weights found");
    }

    // Return total weight
    let child_sum: i32 = child_results.iter().sum();
    Ok(tower.weight + child_sum)
}

/// Calculate bottom element of a tree given each item and its children
pub fn star1(input: &str) -> String {
    resolve_towers(parse_tower_list(input)).name
}

/// Check weights of the tower and return the corrected weight
pub fn star2(input: &str) -> String {
    match check_tower(&resolve_towers(parse_tower_list(input))) {
        Ok(_) => panic!("tower was correct !?"),
        Err(value) => value.to_string(),
    }
}
