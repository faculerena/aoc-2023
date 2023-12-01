use crate::{input_string, input_handler};
use std::io::Read;
pub fn run() -> String {
    let input = input_string!();
    println!("Input: {}", input);
    input
}
