use crate::{Solution, SolutionPair};
use std::{
    fs::{read_to_string, File},
    io::BufReader,
};

///////////////////////////////////////////////////////////////////////////////
///

#[derive(Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => panic!(),
        }
    }

    fn score(&self) -> i64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn win_shape(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn lose_shape(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
}

#[derive(Debug)]
enum GameResult {
    Win,
    Loss,
    Draw,
}

impl GameResult {
    fn score(&self) -> i64 {
        match self {
            GameResult::Win => 6,
            GameResult::Loss => 0,
            GameResult::Draw => 3,
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'X' => GameResult::Loss,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => panic!(),
        }
    }
}

fn shapes_from_line(line: &str) -> (Shape, Shape) {
    let mut chars = line.chars();

    let c1 = chars.next().unwrap();
    chars.next(); // skip the space
    let c2 = chars.next().unwrap();

    (Shape::from_char(c1), Shape::from_char(c2))
}

fn shape_res_pair_from_line(line: &str) -> (Shape, GameResult) {
    let mut chars = line.chars();

    let c1 = chars.next().unwrap();
    chars.next(); // skip the space
    let c2 = chars.next().unwrap();

    (Shape::from_char(c1), GameResult::from_char(c2))
}

fn play_game(opponent: &Shape, me: &Shape) -> GameResult {
    match (opponent, me) {
        (Shape::Rock, Shape::Paper) => GameResult::Win,
        (Shape::Rock, Shape::Scissors) => GameResult::Loss,
        (Shape::Paper, Shape::Rock) => GameResult::Loss,
        (Shape::Paper, Shape::Scissors) => GameResult::Win,
        (Shape::Scissors, Shape::Rock) => GameResult::Win,
        (Shape::Scissors, Shape::Paper) => GameResult::Loss,
        _ => GameResult::Draw,
    }
}

fn calculate_total_score_v1(shape_pairs: &[(Shape, Shape)]) -> i64 {
    shape_pairs
        .iter()
        .map(|(op_shape, me_shape)| {
            let game_result = play_game(op_shape, me_shape);

            game_result.score() + me_shape.score()
        })
        .sum()
}

fn calculate_total_score_v2(shape_result_pair: &[(Shape, GameResult)]) -> i64 {
    shape_result_pair
        .iter()
        .map(|(op_shape, wanted_result)| {
            let me_shape = match wanted_result {
                GameResult::Win => op_shape.win_shape(),
                GameResult::Loss => op_shape.lose_shape(),
                GameResult::Draw => op_shape.clone(),
            };

            wanted_result.score() + me_shape.score()
        })
        .sum()
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let file_string = read_to_string("inputs/day02").unwrap();
    let lines = file_string.split('\n');

    let line_iter = lines.into_iter();

    // let shape_pairs: Vec<(Shape, Shape)> = line_iter
    //     .filter(|line| !line.is_empty())
    //     .map(shapes_from_line)
    //     .collect();

    // let sol1 = calculate_total_score_v1(&shape_pairs);
    let sol1 = 0;

    let shape_result_pairs: Vec<(Shape, GameResult)> = line_iter
        .filter(|line| !line.is_empty())
        .map(shape_res_pair_from_line)
        .collect();
    let sol2 = calculate_total_score_v2(&shape_result_pairs);

    (Solution::I64(sol1), Solution::I64(sol2))
}
