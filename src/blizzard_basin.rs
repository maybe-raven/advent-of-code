//! Day 24: Blizzard Basin
//! https://adventofcode.com/2022/day/24

// #![allow(unused, dead_code)]
use std::{collections::HashSet, convert::identity, fs, str::FromStr};

trait CheckedDec {
    fn checked_dec(self) -> Self;
}

impl CheckedDec for usize {
    fn checked_dec(self) -> Self {
        if self == 0 {
            self
        } else {
            self - 1
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HazzardMovement {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for HazzardMovement {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Self::Right),
            '^' => Ok(Self::Up),
            '<' => Ok(Self::Left),
            'v' => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

// struct Hazzard {
//     movement: Movement,
//     coordinate: Coordinate,
// }
//
// impl Hazzard {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate<const WIDTH: usize, const HEIGHT: usize> {
    x: usize,
    y: usize,
}

impl<const WIDTH: usize, const HEIGHT: usize> Coordinate<WIDTH, HEIGHT> {
    const MAXX: usize = WIDTH - 1;
    const MAXY: usize = HEIGHT - 1;

    const MAX: Self = Self {
        x: Self::MAXX,
        y: Self::MAXY,
    };

    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn new_checked(x: usize, y: usize) -> Option<Self> {
        if x > Self::MAXX || y > Self::MAXY {
            None
        } else {
            Some(Self { x, y })
        }
    }

    fn get_neighbors(&self) -> [Option<Self>; 4] {
        [
            if self.x == 0 {
                None
            } else {
                Some(Self {
                    x: self.x - 1,
                    y: self.y,
                })
            },
            if self.y == 0 {
                None
            } else {
                Some(Self {
                    x: self.x,
                    y: self.y - 1,
                })
            },
            if self.x == WIDTH - 1 {
                None
            } else {
                Some(Self {
                    x: self.x + 1,
                    y: self.y,
                })
            },
            if self.y == HEIGHT - 1 {
                None
            } else {
                Some(Self {
                    x: self.x,
                    y: self.y + 1,
                })
            },
        ]
    }

    fn move_hazzard(self, m: HazzardMovement) -> Self {
        match m {
            HazzardMovement::Up => match self.y {
                0 => Self {
                    y: HEIGHT - 1,
                    ..self
                },
                y => Self { y: y - 1, ..self },
            },
            HazzardMovement::Down => {
                let y = self.y + 1;
                if y == HEIGHT {
                    Self { y: 0, ..self }
                } else {
                    Self { y, ..self }
                }
            }
            HazzardMovement::Left => match self.x {
                0 => Self {
                    x: WIDTH - 1,
                    ..self
                },
                x => Self { x: x - 1, ..self },
            },
            HazzardMovement::Right => {
                let x = self.x + 1;
                if x == WIDTH {
                    Self { x: 0, ..self }
                } else {
                    Self { x, ..self }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board<const WIDTH: usize, const HEIGHT: usize> {
    count: usize,
    hazzards: Vec<(HazzardMovement, Coordinate<WIDTH, HEIGHT>)>,
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    fn tick(&mut self, next_moves: &mut HashSet<Coordinate<WIDTH, HEIGHT>>) {
        for &mut (movement, ref mut coord) in self.hazzards.iter_mut() {
            *coord = coord.move_hazzard(movement);
            next_moves.remove(coord);
        }
        self.count += 1;
    }

    pub fn solve(&mut self) -> usize {
        let entrance = Coordinate::new(0, 0);

        let mut next_moves: HashSet<Coordinate<WIDTH, HEIGHT>> = HashSet::new();

        loop {
            for player_location in next_moves.clone() {
                if player_location == Coordinate::MAX {
                    return self.count + 1;
                }

                next_moves.extend(
                    player_location
                        .get_neighbors()
                        .into_iter()
                        .flat_map(identity),
                );
            }

            next_moves.insert(entrance);

            self.tick(&mut next_moves);
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> FromStr for Board<WIDTH, HEIGHT> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hazzards = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, ch)| {
                    let Some(coord) = Coordinate::new_checked(x, y) else { return Some(Err(())); };
                    let movement = HazzardMovement::try_from(ch).ok()?;
                    Some(Ok((movement, coord)))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { count: 0, hazzards })
    }
}

pub fn main() -> Result<(), String> {
    const INPUT_FILENAME: &str = "input/blizzard_basin.txt";
    let input = fs::read_to_string(INPUT_FILENAME).map_err(|e| e.to_string())?;
    let mut board: Board<100, 35> = input.parse().expect("We choose the correct input here.");
    println!("{}", board.solve());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = ">>.<^<
.<..<<
>v.><>
<^v^^>";

    const TEST_INPUT_1: &str = ".....
 >....
 ...v.";

    const TEST_INPUT_2: &str = "..\n..";

    #[test]
    fn test_board_from_str() {
        let board: Board<6, 4> = TEST_INPUT_0.parse().unwrap();

        let expected_hazzards: [(HazzardMovement, Coordinate<6, 4>); 19] = [
            (HazzardMovement::Right, Coordinate::new(0, 0)),
            (HazzardMovement::Right, Coordinate::new(1, 0)),
            (HazzardMovement::Left, Coordinate::new(3, 0)),
            (HazzardMovement::Up, Coordinate::new(4, 0)),
            (HazzardMovement::Left, Coordinate::new(5, 0)),
            (HazzardMovement::Left, Coordinate::new(1, 1)),
            (HazzardMovement::Left, Coordinate::new(4, 1)),
            (HazzardMovement::Left, Coordinate::new(5, 1)),
            (HazzardMovement::Right, Coordinate::new(0, 2)),
            (HazzardMovement::Down, Coordinate::new(1, 2)),
            (HazzardMovement::Right, Coordinate::new(3, 2)),
            (HazzardMovement::Left, Coordinate::new(4, 2)),
            (HazzardMovement::Right, Coordinate::new(5, 2)),
            (HazzardMovement::Left, Coordinate::new(0, 3)),
            (HazzardMovement::Up, Coordinate::new(1, 3)),
            (HazzardMovement::Down, Coordinate::new(2, 3)),
            (HazzardMovement::Up, Coordinate::new(3, 3)),
            (HazzardMovement::Up, Coordinate::new(4, 3)),
            (HazzardMovement::Right, Coordinate::new(5, 3)),
        ];

        assert_eq!(
            expected_hazzards.len(),
            board.hazzards.len(),
            "Expected {} hazzards, but got {} hazzards",
            expected_hazzards.len(),
            board.hazzards.len()
        );
        for hazzard in expected_hazzards {
            assert!(
                board.hazzards.contains(&hazzard),
                "Parsed board should contain the hazzard {:?}",
                hazzard
            );
        }
    }

    #[test]
    fn test_board_solve_0() {
        let mut board: Board<6, 4> = TEST_INPUT_0.parse().unwrap();

        let result = board.solve();
        assert_eq!(18, result, "The expected solution is 18. Got {}", result);
    }

    #[test]
    fn test_board_solve_1() {
        let mut board: Board<5, 3> = TEST_INPUT_1.parse().unwrap();

        let result = board.solve();
        assert_eq!(8, result, "The expected solution is 8. Got {}", result);
    }

    #[test]
    fn test_board_solve_2() {
        let mut board: Board<2, 2> = TEST_INPUT_2.parse().unwrap();

        let result = board.solve();
        assert_eq!(4, result, "Hazzards in the board: {:?}", board.hazzards);
    }
}
