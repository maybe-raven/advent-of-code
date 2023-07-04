#![allow(dead_code)]
use std::io;

pub fn main() -> Result<(), String> {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .map_err(|e| e.to_string())?;

    if !line.is_ascii() {
        return Err("non-ascii characters are not supported.".to_owned());
    }

    let result = start_of_message(&line).ok_or("marker not found")?;
    println!("{}", result);

    Ok(())
}

fn start_of_packet(s: &str) -> Option<usize> {
    const WINDOW_SIZE: usize = 4;

    s.as_bytes()
        .windows(WINDOW_SIZE)
        .enumerate()
        .find_map(|(i, window)| {
            let &[a, b, c, d] = window else { unreachable!() };
            if a == b || a == c || a == d || b == c || b == d || c == d {
                None
            } else {
                Some(i)
            }
        })
        .map(|x| x + WINDOW_SIZE)
}

fn start_of_message(s: &str) -> Option<usize> {
    const WINDOW_SIZE: usize = 14;

    s.as_bytes()
        .windows(WINDOW_SIZE)
        .enumerate()
        .find_map(|(i, window)| {
            for j in 0..window.len() - 1 {
                let x = window[j];
                if window[j + 1..].contains(&x) {
                    return None;
                }
            }

            Some(i)
        })
        .map(|x| x + WINDOW_SIZE)
}
