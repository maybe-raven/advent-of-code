use std::{num::NonZeroUsize, ops::Deref, str::CharIndices};

const DIGIT_WORDS: [(&str, usize); 10] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

struct MyIter<'s> {
    e: Option<NonZeroUsize>,
    s: &'s str,
    ci: CharIndices<'s>,
}

impl Iterator for MyIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((i, c)) = self.ci.next() {
            if let Some(d) = c.to_digit(10) {
                return Some(d as usize);
            } else if let Some((s, d)) = DIGIT_WORDS
                .into_iter()
                .find_map(|(word, digit)| Some((self.s[i..].strip_prefix(word)?, digit)))
            {
                self.s = s;
                self.ci = s.char_indices();
                return Some(d);
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.ci.size_hint();
        if lower == 0 {
            (0, Some(0))
        } else {
            (0, upper)
        }
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.next_back()
    }
}

impl DoubleEndedIterator for MyIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some((i, c)) = self.ci.next_back() {
            if let Some(d) = c.to_digit(10) {
                return Some(d as usize);
            }

            for (word, digit) in DIGIT_WORDS {
                let s = if let Some(e) = self.e {
                    &self.s[..e.get()]
                } else {
                    self.s
                };

                if let Some(s) = s.strip_suffix(word) {
                    self.e = None;
                    self.s = s;
                    self.ci = s.char_indices();
                    return Some(digit);
                }
            }

            // If `i == 0`, that means this is the first character, which means `ci` has been
            // exhausted, which means going backwards this is the last iteration so the fact that
            // we lose the end index here doesn't matter.
            self.e = NonZeroUsize::new(i);
        }

        None
    }
}

trait IntoMyIter {
    fn my_iter(&self) -> MyIter;
}

impl<T: Deref<Target = str>> IntoMyIter for T {
    fn my_iter(&self) -> MyIter {
        MyIter {
            e: None,
            s: self,
            ci: self.char_indices(),
        }
    }
}

fn main() -> Result<(), String> {
    let answer: Result<usize, String> = std::io::stdin()
        .lines()
        .map(|input| -> Result<usize, String> {
            let line = input.map_err(|e| e.to_string())?;

            let mut iter = line.my_iter();
            // Problem description doesn't specify what to do if the input has no digit, so assume
            // the input is invalid.
            let first_digit = iter
                .next()
                .ok_or_else(|| format!("No digit found in line: {}", line))?;
            // Problem description does specify that if the second digit is missing, the first
            // digit is used twice.
            let last_digit = iter.last().unwrap_or(first_digit);

            Ok(first_digit * 10 + last_digit)
        })
        .sum();

    println!("{}", answer?);

    Ok(())
}
