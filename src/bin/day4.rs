use std::collections::BTreeSet;

fn process_line(s: &str) -> Result<usize, String> {
    let aux = || {
        let (_, s) = s.split_once(':')?;
        let (a, b) = s.split_once('|')?;

        let a: BTreeSet<&str> = a.split_whitespace().collect();
        let n = b.split_whitespace().filter(|&x| a.contains(x)).count();

        if n == 0 {
            Some(0)
        } else {
            Some(1 << (n - 1))
        }
    };

    aux().ok_or_else(|| format!("Failed to parse line: {s}"))
}

#[test]
fn test_process_line_0() {
    assert_eq!(
        Ok(8),
        process_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
    );
}

#[test]
fn test_process_line_1() {
    assert_eq!(
        Ok(2),
        process_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19")
    );
}

#[test]
fn test_process_line_2() {
    assert_eq!(
        Ok(1),
        process_line("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83")
    );
}

#[test]
fn test_process_line_3() {
    assert_eq!(
        Ok(0),
        process_line("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")
    );
}

fn solutionate(iter: impl Iterator<Item = Result<String, impl ToString>>) -> Result<usize, String> {
    iter.map(|line| process_line(line.map_err(|e| e.to_string())?.as_str()))
        .sum()
}

fn main() -> Result<(), String> {
    let answer = solutionate(std::io::stdin().lines())?;
    println!("{}", answer);
    Ok(())
}
