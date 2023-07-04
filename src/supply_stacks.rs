#![allow(dead_code)]
use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp::max, fmt::Display, io, ops::IndexMut, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item(char);

impl Item {
    fn from_str_opt(s: &str) -> Option<Item> {
        let mut iter = s.chars();

        let a = iter.next()?;
        let b = iter.next()?;
        let c = iter.next()?;

        if a != '[' || c != ']' {
            return None;
        }

        Some(Item(b))
    }
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Item::from_str_opt(s).ok_or_else(|| format!("{} is not a valid `Item`", s))
    }
}

impl From<Item> for char {
    fn from(value: Item) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.quantity, self.from, self.to
        )
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn aux(s: &str) -> Option<Instruction> {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
            }

            let captures = RE.captures(s)?;
            let quantity = captures.get(1)?.as_str().parse().ok()?;
            let from = captures
                .get(2)?
                .as_str()
                .parse::<usize>()
                .ok()?
                .checked_sub(1)?;
            let to = captures
                .get(3)?
                .as_str()
                .parse::<usize>()
                .ok()?
                .checked_sub(1)?;

            Some(Instruction { quantity, from, to })
        }

        aux(s).ok_or_else(|| format!("{} is not a valid instruction", s))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Staging(Vec<Vec<Item>>);

impl Staging {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn process_line(&mut self, s: &str) {
        const STEP: usize = 4;
        const SLICE_LEN: usize = 3;

        let n = (s.len() + 1) / STEP;

        if self.0.len() < n {
            self.0.resize(n, Vec::new());
        }

        let mut i = 0;
        for j in 0..n {
            let Some(str_slice) = s.get(i..i+SLICE_LEN) else { return; };
            if let Some(item) = Item::from_str_opt(str_slice) {
                self.0[j].push(item);
            }
            i += STEP;
        }
    }

    fn process_instruction(&mut self, instruction: Instruction) -> Result<(), String> {
        let Instruction { quantity, from, to } = instruction;

        let max_index = max(from, to);
        if self.0.len() <= max_index {
            return Err(format!(
                "error executing instruction \"{}\": stack {} doesn't exist",
                instruction, max_index
            ));
        }

        let from_stack = self.0.index_mut(from);

        let partition_index = from_stack.len().checked_sub(quantity).ok_or_else(|| {
            format!(
                "error executing instruction \"{}\": stack {} contains only {} items",
                instruction,
                from,
                from_stack.len(),
            )
        })?;

        let stuff_to_move: Vec<Item> = from_stack.drain(partition_index..).collect();
        self.0[to].extend(stuff_to_move);

        Ok(())
    }

    fn get_answer(&self) -> String {
        self.0
            .iter()
            .filter_map(|x| x.last())
            .copied()
            .map(char::from)
            .collect()
    }

    fn finalize_stacks(&mut self) {
        for x in self.0.iter_mut() {
            x.reverse();
        }
    }
}

pub fn main() -> Result<(), String> {
    let iter = io::stdin().lines().map(|x| x.map_err(|e| e.to_string()));
    let result = solutionate(iter)?;
    println!("{}", result);

    Ok(())
}

fn solutionate(mut iter: impl Iterator<Item = Result<String, String>>) -> Result<String, String> {
    let mut staging = Staging::new();

    for line in iter.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        staging.process_line(&line);
    }

    staging.finalize_stacks();

    for line in iter {
        let instruction: Instruction = line?.parse()?;
        staging.process_instruction(instruction)?;
    }

    Ok(staging.get_answer())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_solution() {
        assert_eq!(
            Ok("MCD".to_owned()),
            solutionate(INPUT.lines().map(|x| Ok(x.to_owned())))
        );
    }

    #[test]
    fn test_instruction_from_str() {
        assert_eq!(
            Ok(Instruction {
                quantity: 1,
                from: 1,
                to: 0
            }),
            "move 1 from 2 to 1".parse()
        );
        assert_eq!(
            Ok(Instruction {
                quantity: 3,
                from: 0,
                to: 2
            }),
            "move 3 from 1 to 3".parse()
        );
    }

    #[test]
    fn test_staging_process_instruction() {
        let mut staging = Staging(vec![
            vec![Item('Z'), Item('N')],
            vec![Item('M'), Item('C'), Item('D')],
            vec![Item('P')],
        ]);
        assert!(staging
            .process_instruction(Instruction {
                quantity: 1,
                from: 1,
                to: 0,
            })
            .is_ok());
        assert_eq!(
            Staging(vec![
                vec![Item('Z'), Item('N'), Item('D')],
                vec![Item('M'), Item('C')],
                vec![Item('P')],
            ]),
            staging
        );
    }
}
