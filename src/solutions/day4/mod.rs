use crate::{input_handler, input_string};
use std::collections::HashMap;
use std::io::Read;

pub fn run1() -> String {
    let input = input_string!();
    let mut sum = 0;

    for line in input.lines() {
        let line_parts: Vec<Vec<u32>> = line[9..]
            .split("|")
            .map(|x| {
                x.trim()
                    .replace("  ", " ")
                    .split(" ")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        let mut our_values = line_parts[0].clone();
        our_values.retain(|x| line_parts[1].contains(x));

        if our_values.len() == 0 {
            continue;
        } else {
            sum += 2_i32.pow(our_values.len() as u32 - 1);
        }
    }
    sum.to_string()
}

pub fn run2() -> String {
    let input = input_string!();

    let mut card_storage: HashMap<u32, u32> = (1..=199).map(|i| (i, 1)).collect();

    for (i, line) in input.lines().enumerate() {
        let line_parts: Vec<Vec<u32>> = line[9..]
            .split("|")
            .map(|x| {
                x.trim()
                    .replace("  ", " ")
                    .split(" ")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        let mut our_values = line_parts[0].clone();
        our_values.retain(|x| line_parts[1].contains(x));

        for _ in 0..card_storage.get(&(i as u32 + 1)).unwrap().clone() {
            for j in 1..=our_values.len() as u32 {
                let next_card = card_storage.get_mut(&(j + i as u32 + 1)).unwrap();
                *next_card += 1;
            }
        }
    }

    card_storage.values().map(|x| x).sum::<u32>().to_string()
}
