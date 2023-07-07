#![allow(dead_code)]
use std::{fmt::Display, io, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Copy)]
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

struct Tele {
    register: i32,
    cycle_id: i32,
    output: Vec<char>,
}

impl Tele {
    fn new() -> Self {
        Self {
            register: 1,
            cycle_id: 0,
            output: Vec::with_capacity(40),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => self.draw(),
            Instruction::AddX(x) => {
                self.draw();
                self.draw();
                self.register += x;
            }
        }
    }

    fn draw(&mut self) {
        let ch = if (self.cycle_id % 40 - self.register).abs() < 2 {
            '#'
        } else {
            '.'
        };
        self.output.push(ch);
        self.cycle_id += 1;
    }
}

impl Display for Tele {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .output
            .chunks(40)
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        f.write_str(s.as_str())
    }
}

pub fn main() -> Result<(), String> {
    let iter = io::stdin().lines();
    let mut tele = Tele::new();

    for input in iter {
        let instruction = input.map_err(|e| e.to_string())?.parse()?;
        tele.execute(instruction);
    }
    println!("{}", tele);

    Ok(())
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

    const OUTPUT: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn test_solution() {
        let mut tele = Tele::new();
        for line in INPUT.lines() {
            let instruction = line.parse().unwrap();
            tele.execute(instruction);
        }
        let result = tele.to_string();
        assert_eq!(OUTPUT, result, "\nexpect:\n{}\ngot:\n{}\n", OUTPUT, result);
    }
}
