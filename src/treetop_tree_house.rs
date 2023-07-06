#![allow(dead_code)]
use std::{
    fmt::{Display, Write},
    io::{self, Read},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    height: u8,
    score: usize,
}

impl Tile {
    fn new(height: u8) -> Self {
        Self { height, score: 1 }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Tile::new(value.to_digit(10).ok_or(())? as u8))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Board(Vec<Vec<Tile>>);

impl Board {
    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0.first().map(|x| x.len()).unwrap_or(0)
    }

    fn solutionate(&mut self) -> usize {
        let board_width = self.width();
        let board_height = self.height();

        let mut max_score = 0;

        for x in 0..board_width {
            for y in 0..board_height {
                let tree_height = self.0[y][x].height;

                let mut score = 1;
                score *= (1..=y)
                    .find(|offset| tree_height <= self.0[y - offset][x].height)
                    .unwrap_or(y);

                score *= (1..=x)
                    .find(|offset| tree_height <= self.0[y][x - offset].height)
                    .unwrap_or(x);

                let max_down_offset = board_height - y - 1;
                score *= (1..max_down_offset)
                    .find(|offset| tree_height <= self.0[y + offset][x].height)
                    .unwrap_or(max_down_offset);

                let max_right_offset = board_width - x - 1;
                score *= (1..max_right_offset)
                    .find(|offset| tree_height <= self.0[y][x + offset].height)
                    .unwrap_or(max_right_offset);

                if max_score < score {
                    max_score = score;
                }

                self.0[y][x].score = score;
            }
        }

        max_score
    }
}

impl FromStr for Board {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map_line = |line: &str| {
            line.chars()
                .map(Tile::try_from)
                .collect::<Result<_, ()>>()
                .map_err(|_| "input contains invalid character")
        };

        if s.is_empty() {
            return Err("input is empty");
        }

        let mut width = None;
        let data = s
            .lines()
            .map(|line| {
                if let Some(w) = width {
                    if w == line.len() {
                        map_line(line)
                    } else {
                        Err("input is not a grid.")
                    }
                } else {
                    width = Some(line.len());
                    map_line(line)
                }
            })
            .collect::<Result<_, _>>()?;

        Ok(Self(data))
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| tile.height.to_string())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        f.write_str("grid: \n")?;
        f.write_str(s.as_str())?;
        f.write_char('\n')?;

        let v = self
            .0
            .iter()
            .map(|row| row.iter().map(|tile| tile.score).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        f.write_str("scores: \n")?;
        write!(f, "{:?}", v)
    }
}

pub fn main() -> Result<(), String> {
    let mut s = String::new();
    io::stdin()
        .read_to_string(&mut s)
        .map_err(|e| e.to_string())?;

    let mut board: Board = s.parse()?;
    println!("{}", board.solutionate());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_board_from_str() {
        assert_eq!(
            Ok(Board(vec![
                vec![
                    Tile::new(3),
                    Tile::new(0),
                    Tile::new(3),
                    Tile::new(7),
                    Tile::new(3)
                ],
                vec![
                    Tile::new(2),
                    Tile::new(5),
                    Tile::new(5),
                    Tile::new(1),
                    Tile::new(2)
                ],
                vec![
                    Tile::new(6),
                    Tile::new(5),
                    Tile::new(3),
                    Tile::new(3),
                    Tile::new(2)
                ],
                vec![
                    Tile::new(3),
                    Tile::new(3),
                    Tile::new(5),
                    Tile::new(4),
                    Tile::new(9)
                ],
                vec![
                    Tile::new(3),
                    Tile::new(5),
                    Tile::new(3),
                    Tile::new(9),
                    Tile::new(0)
                ]
            ])),
            INPUT.parse()
        );
    }

    #[test]
    fn test_solution() {
        let mut board: Board = INPUT.parse().unwrap();
        assert_eq!(8, board.solutionate());
    }
}
