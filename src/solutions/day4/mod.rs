use crate::{input_handler, input_string};
use std::io::Read;

pub fn run1() -> String {
    let input = input_string!();
    let mut sum = 0;

    for line in input.lines() {
        let line_parts: Vec<Vec<u32>> = line[9..]
            .split("|")
            .map(|x| x.trim().split(" ").filter(|x| x.len() > 0).collect())
            .collect();

        let mut our_values = line_parts[0].clone();
        our_values.retain(|x| line_parts[1].contains(x));

        if our_values.len() == 0 {
            continue;
        } else {
            sum += 1 << (our_values.len() as u32 - 1);
        }
    }
    sum.to_string()
}

pub fn run2() -> String {
    let input = input_string!();
    let mut card_storage = [1; 199];

    for (i, line) in input.lines().enumerate() {
        let line_parts: Vec<Vec<_>> = line[9..]
            .split("|")
            .map(|x| x.trim().split(" ").filter(|x| x.len() > 0).collect())
            .collect();

        let mut our_values = line_parts[0].clone();
        our_values.retain(|x| line_parts[1].contains(x));

        for _ in 0..card_storage[i] {
            for j in 1..=our_values.len() {
                card_storage[j + i] += 1;
            }
        }
    }
    card_storage.iter().sum::<u32>().to_string()
}
