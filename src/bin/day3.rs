use std::{io, mem::swap};

trait IsPartSymbol {
    fn is_part_symbol(&self) -> bool;
}

impl IsPartSymbol for u8 {
    fn is_part_symbol(&self) -> bool {
        !self.is_ascii_alphanumeric() && self != &b'.'
    }
}

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

#[derive(Debug, Clone, Default)]
struct PartSymbols {
    v: [Vec<usize>; 3],
}

impl PartSymbols {
    fn push(&mut self, value: usize) {
        self.v[2].push(value);
    }

    fn newline(&mut self) {
        self.v.swap(0, 1);
        self.v.swap(1, 2);
        self.v[2].clear();
    }

    fn check(&self, part: &PartNumber) -> bool {
        let start = if part.start_index == 0 {
            0
        } else {
            part.start_index - 1
        };

        let end = part.end_index + 1;

        self.v.iter().flatten().any(|i| (start..=end).contains(i))
    }
}

fn main() -> Result<(), String> {
    let mut part_numbers_current = Vec::new();
    let mut part_numbers_scanning = Vec::new();
    let mut part_symbols = PartSymbols::default();
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
        let part_symbols = &mut part_symbols;
        let part_numbers_current = &mut part_numbers_current;
        let part_numbers_scanning = &mut part_numbers_scanning;
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
                    if c.is_part_symbol() {
                        part_symbols.push(i);
                    }

                    if started {
                        part_numbers_scanning.push(PartNumber {
                            start_index,
                            end_index,
                            value,
                        });
                        started = false;
                        value = 0;
                    }
                }

                end_index = i;
            }

            for part in part_numbers_current.iter() {
                if part_symbols.check(part) {
                    *sum += part.value;
                }
            }

            dbg!(&part_symbols);
            dbg!(&part_numbers_current);
            dbg!(&part_numbers_scanning);
            dbg!(&sum);

            part_symbols.newline();
            swap(part_numbers_current, part_numbers_scanning);
            part_numbers_scanning.clear();
        }
    };

    for line in io::stdin().lines() {
        process(preprocess(line)?.as_str());
    }

    process("");

    println!("{}", sum);

    Ok(())
}
