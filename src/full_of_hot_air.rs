//! Day 25: Full of Hot Air
//! https://adventofcode.com/2022/day/25

use std::fs;

fn parse_snafu_digit(value: u8) -> Option<i64> {
    match value {
        b'2' => Some(2),
        b'1' => Some(1),
        b'0' => Some(0),
        b'-' => Some(-1),
        b'=' => Some(-2),
        _ => None,
    }
}

fn snafu_to_i64(input: &str) -> Option<i64> {
    input
        .bytes()
        .rev()
        .enumerate()
        .try_fold(0, |acc, (index, digit)| {
            Some(acc + 5_i64.pow(index as u32) * parse_snafu_digit(digit)?)
        })
}

const INPUT_FILENAME: &'static str = "input/full_of_hot_air.txt";

pub fn main() -> Result<(), String> {
    let input = fs::read_to_string(INPUT_FILENAME).map_err(|e| e.to_string())?;
    let result = input
        .lines()
        .try_fold(0, |acc, line| Some(acc + snafu_to_i64(line)?))
        .ok_or("Invalid SNAFU number.")?;
    println!("{}", result);

    Ok(())
}
