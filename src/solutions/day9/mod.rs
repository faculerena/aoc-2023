use crate::input_string;
use std::io::Read;

pub fn run1() -> String {
    input_string!()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| infer_value_last(&v))
        .sum::<i32>()
        .to_string()
}

pub fn run2() -> String {
    input_string!()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| infer_value_first(&v))
        .sum::<i32>()
        .to_string()
}

fn infer_value_last(v: &Vec<i32>) -> i32 {
    let diff = infer(v);
    let last = v.last().cloned().unwrap();

    if diff.iter().all(|&x| x == 0) {
        last
    } else {
        last + infer_value_last(&diff)
    }
}

fn infer_value_first(v: &Vec<i32>) -> i32 {
    fn infer_value_first_1(v: &Vec<i32>, sign: bool) -> i32 {
        let diff = infer(v);
        let first = v.first().cloned().unwrap() * if sign { 1 } else { -1 };

        if diff.iter().all(|&x| x == 0) {
            first
        } else {
            first + infer_value_first_1(&diff, !sign)
        }
    }
    infer_value_first_1(&v, true)
}

fn infer(v: &Vec<i32>) -> Vec<i32> {
    v.iter()
        .skip(1)
        .zip(v.iter())
        .map(|(a, b)| a - b)
        .collect::<Vec<i32>>()
}
