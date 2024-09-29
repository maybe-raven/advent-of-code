#![allow(unused)]
use std::{io, mem::swap};

trait ToDigit {
    fn to_digit(self) -> Option<usize>;
}

impl ToDigit for u8 {
    fn to_digit(self) -> Option<usize> {
        // Using lazy evaluation here because some characters' byte values are less than the byte
        // value of the character '0', then `self - b'0'` can underflow. Of course, I can use
        // `saturating_sub` or `checked_sub` instead, but more exploration needed to figure out
        // which is actually better.
        #[allow(clippy::unnecessary_lazy_evaluations)]
        self.is_ascii_digit().then(|| (self - b'0') as usize)
    }
}

#[derive(Debug, Clone, Copy)]
struct PartNumber {
    start_index: usize,
    end_index: usize,
    value: usize,
}

impl PartNumber {
    fn new(start_index: usize, end_index: usize, value: usize) -> Self {
        Self {
            start_index: start_index.saturating_sub(1),
            end_index: end_index + 1,
            value,
        }
    }

    fn check(&self, i: usize) -> bool {
        (self.start_index..=self.end_index).contains(&i)
    }
}

#[derive(Debug, Clone)]
struct ScanningWindow<T> {
    v: [Vec<T>; 3],
}

impl<T> Default for ScanningWindow<T> {
    fn default() -> Self {
        Self {
            v: [Vec::default(), Vec::default(), Vec::default()],
        }
    }
}

impl<T> ScanningWindow<T> {
    fn push(&mut self, value: T) {
        self.v[2].push(value);
    }

    fn newline(&mut self) {
        self.v.swap(0, 1);
        self.v.swap(1, 2);
        self.v[2].clear();
    }

    fn middle(&self) -> &Vec<T> {
        &self.v[1]
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.v.iter().flatten()
    }
}

fn main() -> Result<(), String> {
    let mut part_numbers = ScanningWindow::default();
    let mut gear_symbol_indices = ScanningWindow::default();
    let mut sum = 0;

    fn preprocess(line: io::Result<String>) -> Result<String, String> {
        let s = line.map_err(|e| e.to_string())?;
        if s.is_ascii() {
            Ok(s)
        } else {
            Err("Only ASCII characters are accepted".to_string())
        }
    }

    let mut process = {
        let part_numbers = &mut part_numbers;
        let gear_symbol_indices = &mut gear_symbol_indices;
        let sum = &mut sum;

        move |s: &str| {
            let mut started = false;
            let mut value = 0;
            let mut start_index = 0;
            let mut end_index = 0;

            for (i, c) in s.bytes().enumerate() {
                if let Some(d) = c.to_digit() {
                    if started {
                        // Accumulating.
                        value *= 10;
                        value += d;
                    } else {
                        // First digit.
                        value += d;
                        start_index = i;
                        started = true;
                    }
                } else {
                    if c == b'*' {
                        gear_symbol_indices.push(i);
                    }

                    if started {
                        part_numbers.push(PartNumber::new(start_index, end_index, value));
                        started = false;
                        value = 0;
                    }
                }

                end_index = i;
            }

            if started {
                part_numbers.push(PartNumber::new(start_index, end_index, value));
            }

            // I wonder if multiple gears are allowed to share numbers. This is not specified.
            for &i in gear_symbol_indices.middle() {
                // Create a filter iterator that emits part numbers adjacent to a gear symbol.
                let mut iter = part_numbers.iter().filter(|&p| p.check(i));
                // Use pattern matching to check that the iterator yields exactly two items.
                if let (Some(a), Some(b), None) = (iter.next(), iter.next(), iter.next()) {
                    *sum += a.value * b.value;
                }
            }

            gear_symbol_indices.newline();
            part_numbers.newline();
        }
    };

    for line in io::stdin().lines() {
        process(preprocess(line)?.as_str());
    }

    process("");

    println!("{}", sum);

    Ok(())
}
