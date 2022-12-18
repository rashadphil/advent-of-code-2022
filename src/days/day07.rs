use crate::{Solution, SolutionPair};
use std::{collections::HashMap, fs::read_to_string, iter::Peekable, path::PathBuf, str::Lines};

use itertools::Itertools;

///////////////////////////////////////////////////////////////////////////////
///

impl Command {
    fn from_line(line: &str) -> Self {
        let split: Vec<&str> = line.split(' ').collect();
        match split[1] {
            "cd" => Command::Cd(split[2].into()),
            "ls" => Command::Ls,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug)]
enum Entry {
    Dir(String),
    File(u64, String),
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_entry(line: &str) -> Entry {
    let mut split = line.split(' ');
    let (first, sec) = (split.next().unwrap(), split.next().unwrap());

    match (first, sec) {
        ("dir", name) => Entry::Dir(name.into()),
        (size_str, name) => Entry::File(size_str.parse().unwrap(), name.into()),
    }
}

fn parse_command(line: &str) -> Command {
    let split: Vec<_> = line.split(' ').collect();
    match split[1] {
        "cd" => Command::Cd(split[2].into()),
        "ls" => Command::Ls,
        _ => unreachable!(),
    }
}

fn parse_line(line: &str) -> Line {
    match line.chars().next().unwrap() {
        '$' => Line::Command(parse_command(line)),
        _ => Line::Entry(parse_entry(line)),
    }
}

fn part1(sizes: &[u64]) -> u64 {
    sizes.iter().filter(|s| **s <= 100000).sum()
}

fn part2(sizes: &[u64]) -> u64 {
    let total_space = 70_000_000;
    let min_free_space = 30_000_000;
    let free_space = total_space - sizes[0];

    let needed_space = min_free_space - free_space;

    *sizes
        .iter()
        .sorted()
        .find(|sz| **sz >= needed_space)
        .unwrap()
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("inputs/day07.txt").unwrap();
    let lines: Vec<Line> = input.lines().map(parse_line).collect();
    let mut sizes = [0; 200];

    // Each index is a level
    // each index in a level is a different directory
    let mut sizes = [0; 200];
    let mut path_folders = [0; 200];
    let mut level = 0;
    let mut folder_ct = 0;

    let mut line_iter = lines.into_iter();

    for line in line_iter {
        match line {
            Line::Command(c) => match c {
                Command::Cd(path) => match path.as_str() {
                    ".." => level -= 1,
                    _ => {
                        path_folders[level] = folder_ct;
                        level += 1;
                        folder_ct += 1;
                    }
                },
                Command::Ls => continue,
            },
            Line::Entry(e) => match e {
                Entry::Dir(_) => continue,
                Entry::File(size, _name) => {
                    for i in 0..level {
                        let folder = path_folders[i];
                        sizes[folder] += size
                    }
                }
            },
        }
    }

    let sol1: u64 = part1(&sizes);
    let sol2: u64 = part2(&sizes);

    (Solution::U64(sol1), Solution::U64(sol2))
}
