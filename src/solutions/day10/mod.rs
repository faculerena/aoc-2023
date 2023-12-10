use crate::input_string;
use itertools::Itertools;
use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};
use std::io::Read;

pub fn run1() -> String {
    let input = input_string!();
    let matrix: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let (start, end) = get_start(&matrix);

    let mut stack = vec![(start, end)];
    let mut dists = BTreeMap::new();
    dists.insert((start, end), 0);

    while let Some((curr_row, curr_col)) = stack.pop() {
        let curr_distance = dists[&(curr_row, curr_col)];
        let curr_type = matrix[curr_row][curr_col];
        for (adj, d_row, d_col) in get_adjacent(&matrix, (curr_row, curr_col)) {
            if is_valid_move(adj, curr_type, d_row, d_col) {
                let new_pos = (
                    (curr_row as i64 + d_row) as usize,
                    (curr_col as i64 + d_col) as usize,
                );
                let new_distance = curr_distance + 1;
                match dists.entry(new_pos) {
                    Entry::Vacant(entry) => {
                        entry.insert(new_distance);
                        stack.push(new_pos);
                    }
                    Entry::Occupied(mut entry) => {
                        if new_distance < *entry.get_mut() {
                            entry.insert(new_distance);
                            stack.push(new_pos);
                        }
                    }
                };
            }
        }
    }

    (*dists.values().max().unwrap()).to_string()
}

fn get_adjacent(
    matrix: &[Vec<char>],
    (row, col): (usize, usize),
) -> impl Iterator<Item = (char, i64, i64)> + '_ {
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let row = row as i64;
    let col = col as i64;
    let max_row = matrix.len() as i64;
    let max_col = matrix[0].len() as i64;

    directions.iter().filter_map(move |&(d_row, d_col)| {
        let new_row = row + d_row;
        let new_col = col + d_col;
        if new_row >= 0 && new_row < max_row && new_col >= 0 && new_col < max_col {
            Some((matrix[new_row as usize][new_col as usize], d_row, d_col))
        } else {
            None
        }
    })
}
fn is_valid_move(adj: char, curr_type: char, d_row: i64, d_col: i64) -> bool {
    match (adj, curr_type, d_row, d_col) {
        ('|', '7' | 'F' | '|' | 'S', 1, 0)
        | ('|', 'L' | 'J' | '|' | 'S', -1, 0)
        | ('-', 'L' | 'F' | '-' | 'S', 0, 1)
        | ('-', '7' | 'J' | '-' | 'S', 0, -1)
        | ('L', '7' | 'F' | '|' | 'S', 1, 0)
        | ('L', '7' | 'J' | '-' | 'S', 0, -1)
        | ('J', '7' | 'F' | '|' | 'S', 1, 0)
        | ('J', 'L' | 'F' | '-' | 'S', 0, 1)
        | ('7', 'L' | 'J' | '|' | 'S', -1, 0)
        | ('7', 'L' | 'F' | '-' | 'S', 0, 1)
        | ('F', 'L' | 'J' | '|' | 'S', -1, 0)
        | ('F', '7' | 'J' | '-' | 'S', 0, -1) => true,
        _ => false,
    }
}
pub fn run2() -> String {
    let input = input_string!();

    let matrix: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let (start, end) = get_start(&matrix);

    let mut stack = vec![(start, end)];
    let mut visited = BTreeSet::new();
    visited.insert((start, end));
    let mut closed_loop = vec![(start as i64, end as i64)];

    while let Some(curr_pos) = stack.pop() {
        let curr_type = matrix[curr_pos.0][curr_pos.1];
        for (adj, d_row, d_col) in get_adjacent(&matrix, curr_pos) {
            if is_valid_move(adj, curr_type, d_row, d_col) {
                let new_pos = (
                    (curr_pos.0 as i64 + d_row) as usize,
                    (curr_pos.1 as i64 + d_col) as usize,
                );
                if visited.insert(new_pos) {
                    stack.push(new_pos);
                    closed_loop.push((new_pos.0 as i64, new_pos.1 as i64));
                }
            }
        }
    }

    let circumference = closed_loop.len() as i64;
    closed_loop.push((start as i64, end as i64));

    let twice_area: i64 = closed_loop
        .into_iter()
        .tuple_windows()
        .map(|((x_0, y_0), (x_1, y_1))| (y_0 + y_1) * (x_0 - x_1))
        .sum();

    (twice_area / 2 - (circumference / 2 - 1)).to_string()
}

pub fn get_start(matrix: &Vec<Vec<char>>) -> (usize, usize) {
    matrix
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter().enumerate().find_map(
                move |(col, &c)| {
                    if c == 'S' {
                        Some((row, col))
                    } else {
                        None
                    }
                },
            )
        })
        .unwrap()
}
