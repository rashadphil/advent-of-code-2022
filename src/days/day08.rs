use itertools::{enumerate, Itertools};
use take_until::TakeUntilExt;

use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////
///
///

pub fn fill_visible(tree_matrix: &Vec<Vec<u32>>, visible_matrix: &mut Vec<Vec<bool>>) {
    let num_rows = tree_matrix.len();
    let num_cols = tree_matrix[0].len();
    for c in 0..num_cols {
        let mut inner_closure = |r: usize, height: u32, max_h: &mut Option<u32>| {
            if let Some(h) = max_h {
                if height > *h {
                    *max_h = Some(height);
                    visible_matrix[r][c] = true;
                }
            } else {
                *max_h = Some(height);
                visible_matrix[r][c] = true;
            }
        };

        let forward = 0..num_rows;
        let backward = (0..num_rows).rev();

        let mut max_h = None;
        for r in forward {
            let height = tree_matrix[r][c];
            inner_closure(r, height, &mut max_h)
        }

        let mut max_h = None;
        for r in backward {
            let height = tree_matrix[r][c];
            inner_closure(r, height, &mut max_h)
        }
    }

    // row by row, forward and backwards
    for (r, row) in enumerate(tree_matrix) {
        let mut inner_closure = |c: usize, height: u32, max_h: &mut Option<u32>| {
            if let Some(h) = max_h {
                if height > *h {
                    *max_h = Some(height);
                    visible_matrix[r][c] = true;
                }
            } else {
                *max_h = Some(height);
                visible_matrix[r][c] = true;
            }
        };

        let forward = enumerate(row);
        let backward = enumerate(row).rev();

        let mut max_h = None;
        for (c, height) in forward {
            inner_closure(c, *height, &mut max_h)
        }

        let mut max_h = None;
        for (c, height) in backward {
            inner_closure(c, *height, &mut max_h)
        }
    }
}

pub fn fill_scenic(tree_matrix: &Vec<Vec<u32>>, scenic_scores: &mut Vec<Vec<u32>>) {
    let num_rows = tree_matrix.len();
    let num_cols = tree_matrix[0].len();
    for r in 0..num_rows {
        for c in 0..num_cols {
            let height = tree_matrix[r][c];

            let up_score = (0..r)
                .rev()
                .take_until(|up_r| tree_matrix[*up_r][c] >= height)
                .count();

            let down_score = (r + 1..num_rows)
                .take_until(|down_r| tree_matrix[*down_r][c] >= height)
                .count();

            let left_score = (0..c)
                .rev()
                .take_until(|left_c| tree_matrix[r][*left_c] >= height)
                .count();

            let right_score = (c + 1..num_cols)
                .take_until(|right_c| tree_matrix[r][*right_c] >= height)
                .count();

            scenic_scores[r][c] = (up_score * down_score * left_score * right_score) as u32;
        }
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("inputs/day08.txt").unwrap();
    let tree_matrix = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let rows = tree_matrix.len();
    let cols = tree_matrix[0].len();

    let mut visible_matrix = vec![vec![false; cols]; rows];
    fill_visible(&tree_matrix, &mut visible_matrix);

    let mut scenic_matrix = vec![vec![0; cols]; rows];
    fill_scenic(&tree_matrix, &mut scenic_matrix);

    let sol1: u64 = visible_matrix.iter().flatten().filter(|b| **b).count() as u64;

    let sol2: u64 = *scenic_matrix.iter().flatten().max().unwrap() as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}
