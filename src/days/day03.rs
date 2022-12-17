use crate::{Solution, SolutionPair};
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

///////////////////////////////////////////////////////////////////////////////

pub fn common_chars(s1: &str, s2: &str, s3: &str) -> HashSet<char> {
    s1.chars()
        .filter(|c| s2.contains(*c) && s3.contains(*c))
        .collect()
}

fn part1(priorities : &HashMap<char, i32>, rucksacks : &Vec<&str>) -> i32 {

    let mut total_priority = 0;

    for sack in rucksacks {
        let line_size = sack.len();
        let first_half = &sack[0..(line_size / 2)];
        let second_half = &sack[(line_size / 2)..];

        let set_one = first_half.chars().collect::<HashSet<char>>();
        let set_two = second_half.chars().collect::<HashSet<char>>();

        let union_iter = set_one.intersection(&set_two);

        total_priority += union_iter
            .map(|char| *priorities.get(char).unwrap())
            .sum::<i32>();
    }

    total_priority

}

fn part2(priorities : &HashMap<char, i32>, rucksacks : &Vec<&str>) -> i32 {

    let mut total_priority = 0;

    let mut rucksacks_iter = rucksacks.iter();

    while let Some(one) = rucksacks_iter.next() {
        let two = rucksacks_iter.next().unwrap();
        let three = rucksacks_iter.next().unwrap();

        let common_chars = common_chars(one, two, three);
        let group_priority: i32 = common_chars
            .iter()
            .map(|c| priorities.get(c).unwrap())
            .sum();
        total_priority += group_priority
    }

    total_priority
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("inputs/day03.txt").unwrap();
    let rucksacks : Vec<&str> = input.split('\n').collect();

    let numbers: Vec<i32> = (1..53).collect();
    let chars = ('a'..='z').chain('A'..='Z');

    let priorities: HashMap<char, i32> = chars.into_iter().zip(numbers).collect();

    let part1 = part1(&priorities, &rucksacks);
    let part2 = part2(&priorities, &rucksacks);

    (Solution::I32(part1), Solution::I32(part2))
}
