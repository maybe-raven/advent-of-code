#![allow(dead_code)]

use std::io::{self, Result};

fn max3(x: usize, (a, b, c): (usize, usize, usize)) -> (usize, usize, usize) {
    if x > a {
        (x, a, b)
    } else if x > b {
        (a, x, b)
    } else if x > c {
        (a, b, x)
    } else {
        (a, b, c)
    }
}

fn sum_segment<I: Iterator<Item = Result<String>>>(iter: &mut I) -> usize {
    iter.map_while(|s| s.ok().and_then(|line| line.parse::<usize>().ok()))
        .sum::<usize>()
}

fn solutionate<I: Iterator<Item = Result<String>>>(iter: I) -> usize {
    let mut iter = iter.peekable();
    let mut acc = (0, 0, 0);

    loop {
        let x = sum_segment(iter.by_ref());
        acc = max3(x, acc);

        if iter.peek().is_none() {
            break;
        }
    }

    let (a, b, c) = acc;
    a + b + c
}

pub fn main() {
    let result = solutionate(io::stdin().lines());
    println!("{}", result);
}
