use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Interval(i64, i64);

impl Interval {
    fn add(self, s: i64) -> Interval {
        Interval(self.0 + s, self.1 + s)
    }

    fn intersect(self, b: Interval) -> Interval {
        Interval(self.0.max(b.0), self.1.min(b.1))
    }

    fn is_empty(self) -> bool {
        self.0 >= self.1
    }
}

#[derive(Clone, Debug)]
struct Map {
    entries: Vec<(Interval, i64)>,
}

impl Map {
    fn parse(s: &str) -> Map {
        let raw_entries = s
            .lines()
            .skip(1)
            .map(|line| {
                let (d, s, l) = line
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                (Interval(s, s + l), d - s)
            })
            .sorted_unstable_by_key(|(r, _)| r.0);

        let mut pos = 0;
        let mut entries = Vec::new();
        for (r, off) in raw_entries {
            if pos < r.0 {
                // Synthesize a gap entry
                entries.push((Interval(pos, r.0), 0));
            }
            entries.push((r, off));
            pos = r.1;
        }

        entries.push((Interval(pos, i64::MAX), 0));
        Map { entries }
    }

    fn apply_range(&self, seeds: Interval) -> impl Iterator<Item = Interval> + '_ {
        self.entries
            .iter()
            .map(move |&(r, off)| r.intersect(seeds).add(off))
            .filter(|&r| !r.is_empty())
    }

    fn apply_ranges<'a>(
        &'a self,
        seeds: impl Iterator<Item = Interval> + 'a,
    ) -> impl Iterator<Item = Interval> + 'a {
        seeds.flat_map(|r| self.apply_range(r))
    }
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<Map>) {
    let mut paras = input.split("\n\n");
    let seeds = paras
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    (seeds, paras.map(Map::parse).collect())
}

pub fn star1(input: &str) -> String {
    let (seeds, maps) = parse_input(input);
    seeds
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |p, map| {
                map.apply_range(Interval(p, p + 1)).next().unwrap().0
            })
        })
        .min()
        .unwrap()
        .to_string()
}

pub fn star2(input: &str) -> String {
    let (seeds, maps) = parse_input(input);
    let seed_ranges = seeds
        .chunks_exact(2)
        .map(|c| Interval(c[0], c[0] + c[1]))
        .collect_vec();
    maps.into_iter()
        .fold(seed_ranges, |p, map| {
            map.apply_ranges(p.into_iter()).collect()
        })
        .into_iter()
        .min_by_key(|r| r.0)
        .unwrap()
        .0
        .to_string()
}
