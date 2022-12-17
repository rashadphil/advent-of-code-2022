use crate::{Solution, SolutionPair};
use std::hash::Hash;
use std::{collections::HashSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////
///

fn has_unique_elems<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Hash + Eq,
{
    let mut hashset = HashSet::new();
    iter.into_iter().all(|x| hashset.insert(x))
}

pub fn part1(chars: &[char]) -> u32 {
    let iter = chars.windows(4);
    for (idx, window) in iter.enumerate() {
        if has_unique_elems(window) {
            return (idx + 4) as u32;
        }
    }
    0
}
pub fn part2(chars: &[char]) -> u32 {
    let iter = chars.windows(14);
    for (idx, window) in iter.enumerate() {
        if has_unique_elems(window) {
            return (idx + 14) as u32;
        }
    }
    0
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("inputs/day06.txt").unwrap();
    let chars: Vec<char> = input.chars().collect();

    let sol1 = part1(&chars);
    let sol2 = part2(&chars);

    (Solution::U32(sol1), Solution::U32(sol2))
}
