#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::{ops::IndexMut, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Operation {
    MulSelf,
    Mul(usize),
    Add(usize),
}

impl Operation {
    fn new(s: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Operation: new = old [+*] \w+").unwrap();
        }

        let captures = RE.captures(s)?;
        let op_type = captures.get(1)?.as_str();
        let variable = captures.get(2)?.as_str();

        let variable = if variable == "old" {
            return Some(Self::MulSelf);
        } else {
            variable.parse::<usize>().ok()?
        };

        match op_type {
            "*" => Some(Self::Mul(variable)),
            "+" => Some(Self::Add(variable)),
            _ => None,
        }
    }

    fn exec(&self, value: usize) -> usize {
        match self {
            Operation::MulSelf => value * value,
            Operation::Mul(multiplier) => value * multiplier,
            Operation::Add(adder) => value * adder,
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s).ok_or_else(|| format!("invalid operation: {}", s))
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    dividend: usize,
    divisible_next: usize,
    indivisible_next: usize,
}

struct Game {
    monkeys: Vec<Monkey>,
}

impl Game {
    fn tick(&mut self) {
        for i in 0..self.monkeys.len() {
            let monkey_ref = self.monkeys.index_mut(i);
            let monkey = monkey_ref.clone();
            monkey_ref.items.clear();

            for &item in &monkey.items {
                let new_value = monkey.op.exec(item) / 3;
                let next_index = if new_value % monkey.dividend == 0 {
                    monkey.divisible_next
                } else {
                    monkey.indivisible_next
                };
                self.monkeys[next_index].items.push(new_value);
            }
        }
    }
}
