use crate::{input_string, input_handler};
use std::io::{Read};

pub fn run1() -> String {
    let input = input_string!();

    let sum: u32 = input
        .split('\n')
        .filter_map(|line| {
            let digits: String = line.chars().filter(|c| c.is_numeric()).collect();
            let first_char = digits.chars().nth(0)?.to_digit(10)?;
            let last_char = digits.chars().last()?.to_digit(10)?;
            Some(first_char * 10 + last_char)
        })
        .sum();

    sum.to_string()
}
pub fn run2() -> String {

    input_string!()
        .split('\n')
        .filter_map(|line| {
            let digits : Vec<u64> = line.replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .flat_map(|c| c.to_digit(10))
                .map(|digit| digit as u64)
                .collect();

            let first_char = digits[0] as u32;
            let last_char = digits[digits.len() - 1] as u32;
            Some(first_char * 10 + last_char)
        })
        .sum::<u32>().to_string()

}

/*
pub fn run3() -> String {

    // this was the first attempt to the 2nd part.
    let input = input_string!();
    let mut sum = 0;

    for line in input.split("\n") {

        let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        let mut apparitions = [(-1,-1);9];

        for (i, num) in nums.iter().enumerate() {
            let re = Regex::new(num).unwrap();
            let mut first = -1;
            let mut last = -1;
            for (j, cap) in re.captures_iter(line).enumerate() {
                if j == 0 {
                    first = cap.get(0).unwrap().start() as i32;
                }
                last = cap.get(0).unwrap().end() as i32;
            }

            let re = Regex::new(&string_to_int(num).to_string()).unwrap();
            for (j, cap) in re.captures_iter(line).enumerate() {
                if j == 0 {
                    if (cap.get(0).unwrap().start() as i32) < first || first == -1 {
                        first = cap.get(0).unwrap().start() as i32;
                    }
                }
                if (cap.get(0).unwrap().end() as i32) > last || last == -1 {
                    last = cap.get(0).unwrap().end() as i32;
                }

            }
            apparitions[i] = (first, last);
        }

        let fst = apparitions.iter().enumerate().filter(|(_, &x)| x.0 != -1).min_by_key(|(_, &x)| x.0).unwrap().0 + 1;
        let snd = apparitions.iter().enumerate().max_by_key(|(_, &x)| x.1).unwrap().0 + 1;

        let val = fst * 10 + snd;

        sum += val;
    }

    sum.to_string()
}

fn string_to_int(s: &str) -> u64 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => s.parse::<u64>().unwrap(),
    }
}
*/