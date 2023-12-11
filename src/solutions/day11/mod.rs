use crate::{input_string, test_input};
use itertools::Itertools;
use std::io::Read;

pub fn run1() -> String {
    calculate_expansion_by_factor(2)
}

pub fn run2() -> String {
    calculate_expansion_by_factor(1_000_000)
}

fn calculate_expansion_by_factor(rate: u32) -> String {
    let input = input_string!();

    let mut cols_with_galaxies = Vec::new();
    let mut rows_with_galaxies = Vec::new();
    let mut galaxies: Vec<(i32, i32)> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                cols_with_galaxies.push(j);
                rows_with_galaxies.push(i);
                galaxies.push((i as i32, j as i32));
            }
        }
    }
    let cols_with_galaxies: Vec<_> = cols_with_galaxies.iter().sorted().dedup().collect();
    let rows_with_galaxies: Vec<_> = rows_with_galaxies.iter().sorted().dedup().collect();

    let mut galaxy_pairs: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            galaxy_pairs.push((galaxies[i], galaxies[j]));
        }
    }

    galaxy_pairs
        .iter()
        .map(|&((ax, ay), (bx, by))| {
            let mut rows_between =
                ((std::cmp::min(ax, bx) + 1)..std::cmp::max(ax, bx)).collect::<Vec<_>>();
            let mut cols_between =
                ((std::cmp::min(ay, by) + 1)..std::cmp::max(ay, by)).collect::<Vec<_>>();

            cols_between.retain(|c| !cols_with_galaxies.contains(&&(*c as usize)));
            rows_between.retain(|r| !rows_with_galaxies.contains(&&(*r as usize)));

            ax.abs_diff(bx) as u128
                + ay.abs_diff(by) as u128
                + (cols_between.len() as u128 + rows_between.len() as u128) * (rate - 1) as u128
        })
        .sum::<u128>()
        .to_string()
}
