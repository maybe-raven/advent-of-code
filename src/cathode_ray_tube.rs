#![allow(dead_code)]
use std::{cmp::Ordering, io, num::ParseIntError, str::FromStr};

enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        match iter.next().ok_or("empty input")? {
            "noop" => Ok(Self::Noop),
            "addx" => Ok(Self::AddX(
                iter.next()
                    .ok_or_else(|| format!("incomplete instruction: {}", s))?
                    .parse()
                    .map_err(|e: ParseIntError| e.to_string())?,
            )),
            _ => Err(format!("invalid instruction: {}", s)),
        }
    }
}

pub fn main() -> Result<(), String> {
    println!("{}", solutionate(io::stdin().lines())?);
    Ok(())
}

fn solutionate<E: ToString, I: Iterator<Item = Result<String, E>>>(iter: I) -> Result<i32, String> {
    const CYCLE_TARGETS: [usize; 6] = [20, 60, 100, 140, 180, 220];

    let mut cycle_target_iter = CYCLE_TARGETS.into_iter();
    let mut cycle_target = cycle_target_iter.next().unwrap();
    let mut cycle_id = 1;
    let mut register = 1;
    let mut answer = 0;

    for input in iter {
        let (new_register, new_cycle_id) = match input.map_err(|e| e.to_string())?.parse()? {
            Instruction::Noop => (register, cycle_id + 1),
            Instruction::AddX(x) => (register + x, cycle_id + 2),
        };

        if let Some(addition) = match new_cycle_id.cmp(&cycle_target) {
            Ordering::Less => None,
            Ordering::Equal => Some(new_register),
            Ordering::Greater => Some(register),
        } {
            answer += addition * cycle_target as i32;
            let Some(next_target) = cycle_target_iter.next() else { return Ok(answer); };
            cycle_target = next_target;
        }

        register = new_register;
        cycle_id = new_cycle_id;
    }

    Err(format!(
        "not enough instructions; current cycle: {}; register: {}; answer: {}",
        cycle_id, register, answer
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_solution() {
        assert_eq!(
            Ok(13140),
            solutionate(INPUT.lines().map(|s| Ok::<String, String>(s.to_owned())))
        );
    }
}
