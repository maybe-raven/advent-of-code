//! Day 24: Blizzard Basin
//! https://adventofcode.com/2022/day/24

#![allow(unused, dead_code)]
use std::{cmp::min, collections::VecDeque, convert::identity, fs, ops::Index, str::FromStr};

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

#[derive(Debug, Clone, Copy)]
enum PlayerMovement {
    Up,
    Down,
    Left,
    Right,
    Wait,
}

impl PlayerMovement {
    const MEMBERS: [PlayerMovement; 5] =
        [Self::Up, Self::Down, Self::Left, Self::Right, Self::Wait];
}

#[derive(Debug, Clone, Copy)]
enum HazzardMovement {
    Up,
    Down,
    Left,
    Right,
}

impl HazzardMovement {
    fn reversed(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate<const WIDTH: usize, const HEIGHT: usize> {
    x: usize,
    y: usize,
}

impl<const WIDTH: usize, const HEIGHT: usize> Coordinate<WIDTH, HEIGHT> {
    const MAX: Self = Self {
        x: WIDTH - 1,
        y: HEIGHT - 1,
    };

    fn new(x: usize, y: usize) -> Self {
        Self {
            x: min(x, WIDTH - 1),
            y: min(y, HEIGHT - 1),
        }
    }

    fn new_unchecked(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn move_player(self, m: PlayerMovement) -> Option<Self> {
        match m {
            PlayerMovement::Up => match self.y {
                0 => None,
                y => Some(Self { y: y - 1, ..self }),
            },
            PlayerMovement::Down => {
                let y = self.y + 1;
                if y == HEIGHT {
                    None
                } else {
                    Some(Self { y, ..self })
                }
            }
            PlayerMovement::Left => match self.x {
                0 => None,
                x => Some(Self { x: x - 1, ..self }),
            },
            PlayerMovement::Right => {
                let x = self.x + 1;
                if x == WIDTH {
                    None
                } else {
                    Some(Self { x, ..self })
                }
            }
            PlayerMovement::Wait => Some(self),
        }
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

struct Board<const WIDTH: usize, const HEIGHT: usize> {
    count: usize,
    hazzards: Vec<(HazzardMovement, Coordinate<WIDTH, HEIGHT>)>,
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    fn tick(&mut self) {
        for (movement, coord) in self.hazzards.iter_mut() {
            *coord = coord.move_hazzard(*movement);
        }
        self.count += 1;
    }

    fn rewind(&mut self) {
        for (movement, coord) in self.hazzards.iter_mut() {
            *coord = coord.move_hazzard(movement.reversed());
        }
        self.count -= 1;
    }

    fn tile_is_safe(&self, coord: Coordinate<WIDTH, HEIGHT>) -> bool {
        self.hazzards
            .iter()
            .any(|&(_, hazzard_coord)| hazzard_coord == coord)
    }

    fn solve(&mut self) -> usize {
        let entrance = Coordinate::new_unchecked(0, 0);

        let mut current: VecDeque<(Coordinate<WIDTH, HEIGHT>, [Option<PlayerMovement>; 5])> =
            VecDeque::new();
        let mut next: VecDeque<(Coordinate<WIDTH, HEIGHT>, [Option<PlayerMovement>; 5])> =
            VecDeque::new();

        loop {
            self.tick();

            if self.tile_is_safe(entrance) {
                next.push_back((
                    entrance,
                    [Some(PlayerMovement::Wait), None, None, None, None],
                ));
            }

            for (player_location, movement_possibilities) in current.drain(..) {
                for c in movement_possibilities
                    .into_iter()
                    .filter_map(identity)
                    .filter_map(|m| player_location.move_player(m))
                {
                    if let Some(moves) = self.find_moves(c) {
                        if moves
                            .iter()
                            .copied()
                            .filter_map(identity)
                            .any(|m| c.move_player(m) == Some(Coordinate::MAX))
                        {
                            return self.count;
                        }
                        next.push_back((c, moves));
                    }
                }
            }

            (current, next) = (next, current);
        }
    }

    fn find_moves(
        &self,
        player_location: Coordinate<WIDTH, HEIGHT>,
    ) -> Option<[Option<PlayerMovement>; 5]> {
        let mut movements = PlayerMovement::MEMBERS.map(Some);
        let moved_player_coordinate =
            PlayerMovement::MEMBERS.map(|m| player_location.move_player(m));

        for &(_, hazzard_coordinate) in &self.hazzards {
            let Some(i) = moved_player_coordinate.iter().position(|&c| Some(hazzard_coordinate) == c) else { continue; };
            movements[i] = None;

            if matches!(movements, [None, None, None, None, None]) {
                return None;
            }
        }

        Some(movements)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> FromStr for Board<WIDTH, HEIGHT> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Should check input grid size.
        let hazzards = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, ch)| {
                    HazzardMovement::try_from(ch)
                        .map(move |m| (m, Coordinate::new_unchecked(x, y)))
                        .ok()
                })
            })
            .collect();
        Ok(Self { count: 0, hazzards })
    }
}

// impl<const WIDTH: usize, const HEIGHT: usize> FromStr for Board<WIDTH, HEIGHT> {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         todo!()
//     }
// }

// impl<const WIDTH: usize, const HEIGHT: usize> Index<Coordinate<WIDTH, HEIGHT>>
//     for Board<WIDTH, HEIGHT>
// {
//     type Output = Movement;
//
//     fn index(&self, index: Coordinate<WIDTH, HEIGHT>) -> &Self::Output {
//         &self.0[index.y][index.x]
//     }
// }

pub fn main() -> Result<(), String> {
    const INPUT_FILENAME: &str = "input/blizzard_basin.txt";
    let input = fs::read_to_string(INPUT_FILENAME).map_err(|e| e.to_string())?;
    let mut board: Board<100, 35> = input.parse().expect("We choose the correct input here.");
    println!("{}", board.solve());

    Ok(())
}
