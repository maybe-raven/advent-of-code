use std::collections::HashMap;

fn main() -> Result<(), String> {
    let mut dict = HashMap::with_capacity(10);
    dict.insert("zero", 0);
    dict.insert("one", 1);
    dict.insert("two", 2);
    dict.insert("three", 3);
    dict.insert("four", 4);
    dict.insert("five", 5);
    dict.insert("six", 6);
    dict.insert("seven", 7);
    dict.insert("eight", 8);
    dict.insert("nine", 9);

    let f = |s: &str| -> Option<usize> {
        if let Some(d) = s.chars().next()?.to_digit(10) {
            // If there's at least one character in the string and the first one is a digit,
            // then return that digit as usize.
            Some(d as usize)
        } else {
            // Otherwise, go through `dict` and try to match a word to the start of the string;
            // if there is a match, then return the corresponding digit; otherwise, it's not a
            // digit so return `None`.
            for (&key, &value) in &dict {
                if s.starts_with(key) {
                    return Some(value);
                }
            }
            None
        }
    };

    let answer: Result<usize, String> = std::io::stdin()
        .lines()
        .map(|input| -> Result<usize, String> {
            let line = input.map_err(|e| e.to_string())?;

            if !line.is_ascii() {
                return Err("Only ASCII characters are accepted".to_string());
            }

            let mut iter = (0..line.len()).map(|i| &line[i..]).filter_map(f);
            // Problem description doesn't specify what to do if the input has no digit, so assume
            // the input is invalid.
            let first_digit = iter
                .next()
                .ok_or_else(|| format!("No digit found in line: {}", line))?;
            // Problem description does specify that if the second digit is missing, the first
            // digit is used twice.
            let last_digit = iter.next_back().unwrap_or(first_digit);

            Ok(first_digit * 10 + last_digit)
        })
        .sum();

    println!("{}", answer?);

    Ok(())
}
