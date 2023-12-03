use crate::{input_handler, input_string};
use regex::Regex;
use std::io::Read;
pub fn run1() -> String {
    let input = input_string!();
    let mut sum = 0;
    let lines = input.split("\n").collect::<Vec<&str>>();

    for i in 0..lines.len() {
        let line = lines[i];
        let matches = Regex::new(r"\d+")
            .unwrap()
            .find_iter(line)
            .collect::<Vec<_>>();

        for mc in matches {
            let start = mc.start();
            let end = mc.end();
            let num: u32 = mc.as_str().parse::<u32>().unwrap();
            let adjacent_symbols = get_adjacent_symbols(lines.clone(), i, start, end);
            let filtered = adjacent_symbols
                .iter()
                .filter(|&&x| x != '.' && !x.is_numeric())
                .collect::<Vec<_>>();

            if filtered.len() > 0 {
                sum += num;
            }
        }
    }
    sum.to_string()
}
fn get_adjacent_symbols(lines: Vec<&str>, row: usize, start: usize, end: usize) -> Vec<char> {
    let mut adjacent_symbols = Vec::new();
    for i in row.saturating_sub(1)..=usize::min(row + 1, lines.len() - 1) {
        for j in start.saturating_sub(1)..=usize::min(end, lines[i].len() - 1) {
            let current_char = lines[i].chars().nth(j).unwrap();
            adjacent_symbols.push(current_char);
        }
    }
    adjacent_symbols
}

fn print_lines(lines: &Vec<&str>, i: usize, start: usize, end: usize) {
    let prev = lines[i.saturating_sub(1)]
        [(start.saturating_sub(1))..(usize::min(end + 1, lines.len()))]
        .to_string();
    let next = lines[usize::min(i + 1, lines.len() - 1)]
        [(start.saturating_sub(1))..(usize::min(end + 1, lines.len()))]
        .to_string();
    let act = lines[i][(start.saturating_sub(1))..(usize::min(end + 1, lines.len()))].to_string();
    if act != prev {
        println!("{:?}", prev);
    }
    println!("{:?}", act);
    if act != next {
        println!("{:?}", next);
    }

    println!();
}
pub fn run2() -> String {
    let input = input_string!();
    let mut sum = 0;
    let lines = input.split("\n").collect::<Vec<&str>>();

    for i in 0..lines.len() - 1 {
        let line = lines[i];
        let matches = Regex::new(r"\*")
            .unwrap()
            .find_iter(line)
            .collect::<Vec<_>>();

        for mc in matches {
            let start = mc.start();
            let end = mc.end();
            let adjacent_numbers = get_adjacent_numbers(lines.clone(), i, start, end);

            if adjacent_numbers.len() == 2 {
                sum += adjacent_numbers[0] * adjacent_numbers[1];
            }
        }
    }
    sum.to_string()
}

fn get_adjacent_numbers(lines: Vec<&str>, row: usize, start: usize, end: usize) -> Vec<u32> {
    let mut adjacent_numbers = Vec::new();
    for i in row.saturating_sub(1)..=usize::min(row + 1, lines.len() - 1) {
        for j in start.saturating_sub(1)..=usize::min(end, lines[i].len() - 1) {
            let current_char = lines[i].chars().nth(j).unwrap();
            if current_char.is_numeric() {
                let mut left = j;
                let mut right = j;
                while left > 0 && lines[i].chars().nth(left - 1).unwrap().is_numeric() {
                    left -= 1;
                }
                while right < lines[i].len() - 1
                    && lines[i].chars().nth(right + 1).unwrap().is_numeric()
                {
                    right += 1;
                }
                let num: u32 = lines[i][left..right + 1].parse::<u32>().unwrap();

                adjacent_numbers.push(num);
                if right > j {
                    break;
                }
            }
        }
    }
    adjacent_numbers
}
