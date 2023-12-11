#![allow(dead_code)]
use std::{io, num::ParseIntError};

fn process_line(s: &str) -> isize {
    let Ok(mut nums) = s
        .split_whitespace()
        .map(|x| x.parse())
        .collect::<Result<Vec<isize>, ParseIntError>>()
    else {
        return 0;
    };

    let Some(&last) = nums.last() else {
        return 0;
    };

    let mut last_nums = Vec::new();
    last_nums.push(last);

    while nums.len() > 1 {
        let mut is_same = true;
        let ref_num = nums[1] - nums[0];
        nums[0] = ref_num;
        for i in 1..nums.len() - 1 {
            let new_num = nums[i + 1] - nums[i];
            nums[i] = new_num;
            if new_num != ref_num {
                is_same = false;
            }
        }
        nums.pop();
        if is_same {
            break;
        }
        last_nums.push(*nums.last().expect("`nums` should not be empty."));
    }
    last_nums.push(*nums.last().expect("`nums` should not be empty."));

    last_nums.into_iter().sum()
}

#[test]
fn test_process_line_0() {
    assert_eq!(18, process_line("0 3 6 9 12 15"));
}

#[test]
fn test_process_line_1() {
    assert_eq!(28, process_line("1 3 6 10 15 21"));
}

#[test]
fn test_process_line_2() {
    assert_eq!(68, process_line("10 13 16 21 30 45"));
}

fn main() -> Result<(), io::Error> {
    let answer: isize = io::stdin()
        .lines()
        .map(|line| Ok(process_line(line?.as_str())))
        .sum::<io::Result<isize>>()?;
    println!("{answer}");
    Ok(())
}
