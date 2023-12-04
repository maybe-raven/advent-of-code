use std::collections::BTreeSet;

fn process_line(s: &str) -> Result<usize, String> {
    let aux = || {
        let (_, s) = s.split_once(':')?;
        let (a, b) = s.split_once('|')?;

        let a: BTreeSet<&str> = a.split_whitespace().collect();
        let n = b.split_whitespace().filter(|&x| a.contains(x)).count();

        Some(n)
    };

    aux().ok_or_else(|| format!("Failed to parse line: {s}"))
}

#[derive(Debug, Clone, Default)]
struct Counter {
    v: Vec<usize>,
}

impl Counter {
    fn get(&mut self) -> usize {
        let answer = self.v.len();
        for i in (0..self.v.len()).rev() {
            self.v[i] -= 1;

            if self.v[i] == 0 {
                self.v.swap_remove(i);
            }
        }
        answer
    }

    fn push(&mut self, x: usize, n: usize) {
        if x > 0 {
            for _ in 0..n {
                self.v.push(x);
            }
        }
    }
}

fn solutionate(iter: impl Iterator<Item = Result<String, impl ToString>>) -> Result<usize, String> {
    let mut counter = Counter::default();

    iter.map(|line| {
        let s = line.map_err(|e| e.to_string())?;
        let n_matches = process_line(s.as_str())?;
        let n_copies = counter.get() + 1;
        counter.push(n_matches, n_copies);
        Ok(n_copies)
    })
    .sum()
}

fn main() -> Result<(), String> {
    let answer = solutionate(std::io::stdin().lines())?;
    println!("{}", answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_line_0() {
        assert_eq!(
            Ok(4),
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

    #[test]
    fn test_get_0() {
        let mut counter = Counter {
            v: vec![1, 3, 1, 2, 5, 3],
        };
        assert_eq!(6, counter.get());
        assert_eq!(4, counter.v.len())
    }
}
