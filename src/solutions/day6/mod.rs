use crate::input_string;
use std::io::Read;

pub fn run1() -> String {
    let input = input_string!();

    let result: Vec<u64> = input
        .lines()
        .flat_map(|line| line.split_whitespace().filter_map(|s| s.parse().ok()))
        .collect();

    calculate_posibilites(&result)
}

pub fn run2() -> String {
    let input = input_string!();

    let result = input
        .replace(" ", "")
        .lines()
        .map(|a| {
            a.split(":").collect::<Vec<&str>>()[1]
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    calculate_posibilites(&result)
}

fn calculate_posibilites(arr: &Vec<u64>) -> String {
    let (first_half, second_half) = arr.split_at(arr.len() / 2);

    first_half
        .iter()
        .zip(second_half.iter())
        .collect::<Vec<_>>()
        .iter()
        .fold(1, |acc, run| {
            (0..)
                .find(|&i| i * (run.0 - i) >= *run.1)
                .map_or(acc, |i| acc * (run.0 - 2 * i + 1))
        })
        .to_string()
}
