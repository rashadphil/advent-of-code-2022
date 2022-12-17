use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////
///

pub fn section_nums_from_str(section: &str) -> (i32, i32) {
    let nums: Vec<i32> = section.split('-').map(|s| s.parse().unwrap()).collect();
    (nums[0], nums[1])
}

type SecPair = ((i32, i32), (i32, i32));

pub fn part1(section_pairs: &Vec<SecPair>) -> i32 {
    let mut fully_contain_ct = 0;

    for ((start1, end1), (start2, end2)) in section_pairs {
        if (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1) {
            fully_contain_ct += 1
        }
    }
    fully_contain_ct
}

pub fn part2(section_pairs: &Vec<SecPair>) -> i32 {
    let mut contain_ct = 0;

    for ((start1, end1), (start2, end2)) in section_pairs {
        if (start1 <= end2) && (start2 <= end1) {
            contain_ct += 1
        }
    }
    contain_ct
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("inputs/day04.txt").unwrap();
    let section_pairs = input.split('\n');

    let section_pairs: Vec<SecPair> = section_pairs
        .into_iter()
        .map(|pair| {
            let mut split = pair.split(',');
            let (sec1, sec2) = (split.next().unwrap(), split.next().unwrap());
            (section_nums_from_str(sec1), section_nums_from_str(sec2))
        })
        .collect();

    let sol1 = part1(&section_pairs);
    let sol2 = part2(&section_pairs);

    (Solution::I32(sol1), Solution::I32(sol2))
}
