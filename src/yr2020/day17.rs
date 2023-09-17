use std::collections::HashSet;

use itertools::Itertools;
use nalgebra::SVector;

type VectorI<const D: usize> = SVector<i32, D>;

fn parse<const D: usize>(input: &str) -> HashSet<VectorI<D>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| {
                    let mut v = VectorI::zeros();
                    v[0] = x as i32;
                    v[1] = y as i32;
                    v
                })
        })
        .collect()
}

fn iproduct_vec<const D: usize>(a: VectorI<D>, b: VectorI<D>) -> impl Iterator<Item = VectorI<D>> {
    a.iter()
        .zip(b.iter())
        .map(|(&l, &r)| l..=r)
        .multi_cartesian_product()
        .map(|v| VectorI::from_column_slice(&v))
}

fn step<const D: usize>(old: HashSet<VectorI<D>>) -> HashSet<VectorI<D>> {
    let mut min: VectorI<D> = VectorI::repeat(i32::MAX);
    let mut max: VectorI<D> = VectorI::repeat(i32::MIN);
    for p in old.iter() {
        min = min.zip_map(p, |l, r| l.min(r));
        max = max.zip_map(p, |l, r| l.max(r));
    }

    let one = VectorI::repeat(1);
    let offsets: Vec<_> = iproduct_vec(-one, one)
        .filter(|&v| v != VectorI::<D>::zeros())
        .collect();

    iproduct_vec(min - one, max + one)
        .filter(|&pos| {
            let neighbors = offsets
                .iter()
                .filter(|&off| old.contains(&(pos + off)))
                .count();
            neighbors == 3 || (neighbors == 2 && old.contains(&pos))
        })
        .collect()
}

fn run<const D: usize>(input: &str) -> String {
    (0..6)
        .fold(parse::<D>(input), |grid, _| step(grid))
        .len()
        .to_string()
}

pub fn star1(input: &str) -> String {
    run::<3>(input)
}

pub fn star2(input: &str) -> String {
    run::<4>(input)
}
