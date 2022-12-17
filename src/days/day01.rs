use crate::{Solution, SolutionPair};
use std::{
    cmp::max,
    env,
    fs::{read_to_string, File},
    io::{self, BufRead, BufReader},
};

///////////////////////////////////////////////////////////////////////////////
///

pub fn get_elf_cals(reader: BufReader<File>) -> Vec<i64> {
    let mut result = vec![];

    let mut curr_elf_cals = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        match line.parse::<i64>() {
            Ok(val) => curr_elf_cals += val,
            Err(_) => {
                result.push(curr_elf_cals);
                curr_elf_cals = 0
            }
        }
    }
    result
}

pub fn solve() -> SolutionPair {
    let file = File::open("inputs/day01").unwrap();
    let reader = BufReader::new(file);

    let mut elf_cals = get_elf_cals(reader);
    elf_cals.sort();
    elf_cals.reverse();

    let max_cals = elf_cals[0];
    let three_max_sum = elf_cals.iter().take(3).sum();

    (Solution::I64(max_cals), Solution::I64(three_max_sum))
}
