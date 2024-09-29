#![allow(dead_code)]
use std::io;

pub fn main() -> Result<(), String> {
    let iter = io::stdin().lines().map(|x| x.map_err(|e| e.to_string()));
    let result = solutionate(iter)?;
    println!("{}", result);

    Ok(())
}

fn solutionate(iter: impl Iterator<Item = Result<String, String>>) -> Result<usize, String> {
    let mut count = 0;
    for line in iter {
        if check_line(&line?)? {
            count += 1;
        }
    }
    Ok(count)
}

fn check_line(s: &str) -> Result<bool, String> {
    fn parse_range(s: &str) -> Option<(usize, usize)> {
        let (low, high) = s.split_once('-')?;
        let low: usize = low.parse().ok()?;
        let high: usize = high.parse().ok()?;

        Some((low, high))
    }

    fn aux(s: &str) -> Option<bool> {
        let (a, b) = s.split_once(',')?;
        let (a_low, a_high) = parse_range(a)?;
        let (b_low, b_high) = parse_range(b)?;

        Some(!(a_high < b_low || a_low > b_high))
    }

    aux(s).ok_or_else(|| format!("failed to parse line {}", s))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_check_line() {
        assert_eq!(Ok(true), check_line("2-8,3-7"));
        assert_eq!(Ok(true), check_line("6-6,4-6"));
        assert_eq!(Ok(true), check_line("2-6,4-8"));
        assert_eq!(Ok(false), check_line("2-4,6-8"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(Ok(4), solutionate(INPUT.lines().map(|x| Ok(x.to_owned()))));
    }
}
