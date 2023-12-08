use crate::input_string;
use std::collections::HashMap;
use std::io::Read;

pub fn run1() -> String {
    let input = input_string!();

    let (steps_to_take, graph_part) = input.split_once("\n\n").unwrap();

    let movements = get_movements(graph_part);

    let mut starting_point = "AAA".to_string();
    let mut steps_taken = 0;

    'outer: loop {
        for step in steps_to_take.chars() {
            match step {
                'L' => {
                    starting_point = movements.get(&starting_point).unwrap().clone().0;
                }
                'R' => {
                    starting_point = movements.get(&starting_point).unwrap().clone().1;
                }
                _ => {}
            }
            steps_taken += 1;
            if starting_point == "ZZZ" {
                break 'outer;
            }
        }
    }
    steps_taken.to_string()
}
pub fn run2() -> String {
    let input = input_string!();

    let (steps_to_take, graph_part) = input.split_once("\n\n").unwrap();

    let movements = get_movements(graph_part);

    let mut saved: Vec<_> = movements
        .iter()
        .filter(|(k, _)| k.ends_with("A"))
        .map(|(k, _)| k.clone())
        .take(6)
        .collect();

    let mut steps_taken = [0; 6];

    for i in 0..saved.len() {
        'outer: loop {
            for step in steps_to_take.chars() {
                match step {
                    'L' => {
                        saved[i] = movements.get(&saved[i]).unwrap().clone().0;
                    }
                    'R' => {
                        saved[i] = movements.get(&saved[i]).unwrap().clone().1;
                    }
                    _ => {}
                }
                steps_taken[i] += 1;
                if saved[i].ends_with("Z") {
                    break 'outer;
                }
            }
        }
    }
    fn gcd(mut a: u128, mut b: u128) -> u128 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }

    steps_taken
        .iter()
        .fold(steps_taken[0], |acc, &x| acc * x / gcd(acc, x))
        .to_string()
}

fn get_movements(input: &str) -> HashMap<String, (String, String)> {
    input
        .lines()
        .filter_map(|line| {
            Some((
                line.split_whitespace().nth(0)?.to_string(),
                (
                    line.split_whitespace()
                        .nth(2)?
                        .trim_matches(|c| c == '(' || c == ',' || c == ')')
                        .to_string(),
                    line.split_whitespace()
                        .nth(3)?
                        .trim_matches(|c| c == '(' || c == ',' || c == ')')
                        .to_string(),
                ),
            ))
        })
        .collect()
}
