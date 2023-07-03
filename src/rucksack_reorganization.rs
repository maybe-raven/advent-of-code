#![allow(dead_code)]
use std::{
    array::IntoIter,
    io,
    iter::zip,
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Item(usize);

impl TryFrom<char> for Item {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if ('a'..='z').contains(&value) {
            const LOWER_OFFSET: usize = 'a' as usize;
            Ok(Self(value as usize - LOWER_OFFSET))
        } else if ('A'..='Z').contains(&value) {
            const UPPER_OFFSET: usize = 'A' as usize - 26;
            Ok(Self(value as usize - UPPER_OFFSET))
        } else {
            Err(format!("'{}' is not a valid item", value))
        }
    }
}

impl<T> Index<Item> for [T] {
    type Output = T;

    fn index(&self, index: Item) -> &Self::Output {
        self.index(index.0)
    }
}

impl<T> IndexMut<Item> for [T] {
    fn index_mut(&mut self, index: Item) -> &mut Self::Output {
        self.index_mut(index.0)
    }
}

#[derive(Debug, Clone)]
struct Rucksack([bool; 52]);

impl IntoIterator for Rucksack {
    type Item = bool;

    type IntoIter = IntoIter<bool, 52>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromStr for Rucksack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let indices = s
            .chars()
            .map(Item::try_from)
            .collect::<Result<Vec<Item>, String>>()
            .map_err(|e| format!("failed to parse rucksack {}: {}", s, e))?;

        let mut arr = [false; 52];
        for i in indices {
            arr[i] = true;
        }

        Ok(Self(arr))
    }
}

struct GroupPriorityIter<I> {
    source: I,
}

impl<I: Iterator<Item = Result<Rucksack, String>>> Iterator for GroupPriorityIter<I> {
    type Item = Result<usize, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let a = match self.source.next()? {
            Ok(a) => a,
            Err(e) => return Some(Err(e)),
        };
        let b = match self.source.next() {
            Some(Ok(b)) => b,
            Some(Err(e)) => return Some(Err(e)),
            None => return Some(Err("not enough input for a group".to_owned())),
        };
        let c = match self.source.next() {
            Some(Ok(c)) => c,
            Some(Err(e)) => return Some(Err(e)),
            None => return Some(Err("not enough input for a group".to_owned())),
        };

        Some(
            zip(a, b)
                .zip(c)
                .position(|((a, b), c)| a && b && c)
                .ok_or("no common item found among rucksacks".to_owned())
                .map(|x| x + 1),
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        fn f(n: usize) -> usize {
            if n % 3 == 0 {
                n / 3
            } else {
                n / 3 + 1
            }
        }
        let (lower, upper) = self.source.size_hint();
        (f(lower), upper.map(f))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.source.nth(n * 3 - 1);
        self.next()
    }
}

impl<I: Iterator<Item = Result<Rucksack, String>> + ExactSizeIterator> ExactSizeIterator
    for GroupPriorityIter<I>
{
}

trait GroupPriority<I> {
    fn group_priority(self) -> GroupPriorityIter<I>;
}

impl<I: Iterator> GroupPriority<I> for I {
    fn group_priority(self) -> GroupPriorityIter<I> {
        GroupPriorityIter { source: self }
    }
}

pub fn main() -> Result<(), String> {
    let iter = io::stdin().lines().map(|x| x.map_err(|e| e.to_string()));
    let result = solutionate(iter)?;
    println!("{}", result);

    Ok(())
}

fn solutionate<I: Iterator<Item = Result<String, String>>>(iter: I) -> Result<usize, String> {
    iter.map(|x| x?.parse::<Rucksack>())
        .group_priority()
        .sum::<Result<usize, String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_item_try_from_char() {
        assert_eq!(Ok(Item(28)), Item::try_from('C'));
        assert_eq!(Ok(Item(2)), Item::try_from('c'));
        assert_eq!(Ok(Item(15)), Item::try_from('p'));
        assert!(Item::try_from(' ').is_err());
    }

    #[test]
    fn test_solution() {
        assert_eq!(Ok(70), solutionate(INPUT.lines().map(|x| Ok(x.to_owned()))));
    }
}
