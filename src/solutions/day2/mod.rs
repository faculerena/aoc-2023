use crate::{input_handler, input_string};
use std::collections::HashMap;
use std::io::Read;

pub fn run1() -> String {
    let input = input_string!();

    let mut sum = 0;

    let hm = vec![("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect::<HashMap<&str, u32>>();

    for line in input.lines() {
        let (game_number, parts) = parse_game_string(line);
        let mut valid_game = true;
        for part in parts {
            let draw = part.split(',').collect::<Vec<&str>>();

            for selection in draw.iter() {
                let s = selection.trim().split(' ').collect::<Vec<&str>>();
                let number = s[0].parse::<u32>().unwrap();
                let color = s[1];

                if hm.contains_key(color) && hm.get(color).unwrap() < &number {
                    valid_game = false;
                }
            }
        }
        if valid_game {
            sum += game_number;
        }
    }

    sum.to_string()
}

fn parse_game_string(input: &str) -> (u32, Vec<String>) {
    let iter = input.chars().peekable();

    let game_number: u32 = iter
        .clone()
        .skip(5)
        .take_while(|&c| c != ':')
        .collect::<String>()
        .parse()
        .unwrap();

    let parts: Vec<String> = iter
        .skip_while(|&c| c != ':')
        .skip(1)
        .collect::<String>()
        .split(';')
        .map(|s| s.trim().to_string())
        .collect();

    (game_number, parts)
}

pub fn run2() -> String {
    let input = input_string!();

    let mut sum = 0;

    for line in input.lines() {
        let (_, parts) = parse_game_string(line);

        let mut hm = vec![("red", 0), ("green", 0), ("blue", 0)]
            .iter()
            .cloned()
            .collect::<HashMap<&str, u32>>();

        for part in &parts {
            let draw = part.split(',').collect::<Vec<&str>>();

            for selection in draw.iter() {
                let s = selection.trim().split(' ').collect::<Vec<&str>>();
                let number = s[0].parse::<u32>().unwrap();
                let color = s[1];

                if hm.contains_key(color) && hm.get(color).unwrap() < &number {
                    hm.insert(color, number);
                }
            }
        }
        sum += hm.get("red").unwrap() * hm.get("green").unwrap() * hm.get("blue").unwrap();
    }

    sum.to_string()
}
