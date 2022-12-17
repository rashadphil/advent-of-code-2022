use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

// count, from, to
#[derive(Debug)]
pub struct Instruction(usize, usize, usize);

#[derive(Debug)]
pub struct CargoNCrane {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

enum CraneType {
    CM9000,
    CM9001,
}

pub fn parse_instruction_strings(instruction_strings: &Vec<&str>) -> Vec<Instruction> {
    instruction_strings
        .iter()
        .map(|str| {
            let split: Vec<&str> = str.split(' ').collect();

            let count = split[1].parse().unwrap();
            let from: usize = split[3].parse().unwrap();
            let to: usize = split[5].parse().unwrap();
            Instruction(count, from - 1, to - 1)
        })
        .collect()
}

pub fn parse_crate_strings(crate_strings: &Vec<&str>) -> Vec<Vec<char>> {
    let mut line_iter = crate_strings.iter().rev();

    let nums = line_iter.next().unwrap();
    let nums: Vec<i32> = nums
        .split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect();

    let mut stacks = vec![vec![]; nums.len()];

    for row_str in line_iter {
        for (i, c) in row_str.chars().enumerate() {
            match c {
                ' ' | '[' | ']' => continue,
                _ => {
                    let stack_idx = (i - 1) / 4;
                    stacks[stack_idx].push(c);
                }
            }
        }
    }

    stacks
}

impl CargoNCrane {
    fn from_lines(lines: &Vec<&str>) -> Self {
        let crate_strings: Vec<&str> = lines
            .iter()
            .take_while(|line| !line.is_empty())
            .copied()
            .collect();

        let skip_ct = crate_strings.len() + 1;
        let instruction_strings: Vec<&str> = lines.iter().skip(skip_ct).copied().collect();

        let stacks = parse_crate_strings(&crate_strings);
        let instructions = parse_instruction_strings(&instruction_strings);
        CargoNCrane {
            stacks,
            instructions,
        }
    }

    fn execute_instructions(&mut self, crane_type: CraneType) {
        for Instruction(count, from, to) in &self.instructions {
            let tail: Vec<_> = {
                let from_stack = &mut self.stacks[*from];
                let new_len = from_stack.len() - count;
                let drained = from_stack.drain(new_len..);
                match crane_type {
                    CraneType::CM9000 => drained.rev().collect(),
                    CraneType::CM9001 => drained.collect(),
                }
            };

            let to_stack = &mut self.stacks[*to];
            to_stack.extend(tail);
        }
    }

    fn get_tops(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| *stack.last().unwrap())
            .collect()
    }
}

pub fn part1(lines: &Vec<&str>) -> String {
    let mut cargo_crange = CargoNCrane::from_lines(lines);
    cargo_crange.execute_instructions(CraneType::CM9000);
    cargo_crange.get_tops()
}

pub fn part2(lines: &Vec<&str>) -> String {
    let mut cargo_crange = CargoNCrane::from_lines(lines);
    cargo_crange.execute_instructions(CraneType::CM9001);
    cargo_crange.get_tops()
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("inputs/day05.txt").unwrap();
    let lines = input.lines().collect();

    // Your solution here...
    let sol1 = part1(&lines);
    let sol2 = part2(&lines);

    (Solution::Str(sol1), Solution::Str(sol2))
}
