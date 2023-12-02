use std::str::FromStr;

mod day_1;

#[derive(Debug, Clone, Copy)]
enum Color {
    Blue,
    Red,
    Green,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("blue") {
            Ok(Self::Blue)
        } else if s.eq_ignore_ascii_case("red") {
            Ok(Self::Red)
        } else if s.eq_ignore_ascii_case("green") {
            Ok(Self::Green)
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct CubeCounts {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeCounts {
    fn acc(self, other: Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl FromStr for CubeCounts {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let aux = || -> Option<Self> {
            let mut cube_counts = CubeCounts::default();

            for item in s.split(',') {
                let (count_str, color_str) = item.trim().split_once(' ')?;
                let count = count_str.parse().ok()?;
                match color_str.parse().ok()? {
                    Color::Blue => cube_counts.blue = count,
                    Color::Red => cube_counts.red = count,
                    Color::Green => cube_counts.green = count,
                }
            }

            Some(cube_counts)
        };
        aux().ok_or(())
    }
}

fn main() -> Result<(), String> {
    let answer: Result<usize, String> = std::io::stdin()
        .lines()
        .map(|s| -> Result<usize, String> {
            // Propagate IO Error.
            let s = s.map_err(|e| e.to_string())?;

            // Trim useless ID information. We just use line number instead.
            let (_, s) = s
                .split_once(':')
                .ok_or_else(|| format!("Failed to parse line: {s}"))?;

            let power = s
                .split(';')
                .map(|x| -> Result<CubeCounts, String> {
                    x.trim()
                        .parse()
                        .map_err(|_| format!("Failed to parse item \"{x}\" in input: {s}"))
                })
                .reduce(|acc, x| Ok(acc?.acc(x?)))
                .ok_or_else(|| "Input is empty".to_string())??
                .power();

            Ok(power)
        })
        .sum();

    println!("{}", answer?);

    Ok(())
}
