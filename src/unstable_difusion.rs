//! Day 23: Unstable Diffusion
//! https://adventofcode.com/2022/day/23

#![allow(unused, dead_code)]

use std::{
    collections::{hash_map::OccupiedEntry, VecDeque},
    convert::identity,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    const COUNT: usize = 4;
    const MEMBERS: [Self; Self::COUNT] = [Self::North, Self::South, Self::West, Self::East];

    fn reversed(self) -> Self {
        match self {
            Direction::North => Self::South,
            Direction::South => Self::North,
            Direction::West => Self::East,
            Direction::East => Self::West,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }

    fn transform(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self::new(self.x, self.y - 1),
            Direction::South => Self::new(self.x, self.y + 1),
            Direction::West => Self::new(self.x - 1, self.y),
            Direction::East => Self::new(self.x + 1, self.y),
        }
    }

    fn get_neighbors(&self, direction: Direction) -> [Self; 3] {
        match direction {
            Direction::North => [
                Coordinate::new(self.x - 1, self.y - 1), // NW
                Coordinate::new(self.x, self.y - 1),     // N
                Coordinate::new(self.x + 1, self.y - 1), // NE
            ],
            Direction::South => [
                Coordinate::new(self.x - 1, self.y + 1), // SW
                Coordinate::new(self.x, self.y + 1),     // S
                Coordinate::new(self.x + 1, self.y + 1), // SE
            ],
            Direction::West => [
                Coordinate::new(self.x - 1, self.y - 1), // NW
                Coordinate::new(self.x - 1, self.y),     // W
                Coordinate::new(self.x - 1, self.y + 1), // SW
            ],
            Direction::East => [
                Coordinate::new(self.x + 1, self.y - 1), // NE
                Coordinate::new(self.x + 1, self.y),     // E
                Coordinate::new(self.x + 1, self.y + 1), // SE
            ],
        }
    }

    fn get_all_neighbors(&self) -> [Self; 8] {
        [
            Coordinate::new(self.x - 1, self.y - 1), // NW
            Coordinate::new(self.x, self.y - 1),     // N
            Coordinate::new(self.x + 1, self.y - 1), // NE
            Coordinate::new(self.x - 1, self.y),     // W
            Coordinate::new(self.x + 1, self.y),     // E
            Coordinate::new(self.x - 1, self.y + 1), // SW
            Coordinate::new(self.x, self.y + 1),     // S
            Coordinate::new(self.x + 1, self.y + 1), // SE
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Occupied,
    Proposed(Direction),
    Blocked,
}

struct Board {
    board: VecDeque<VecDeque<Tile>>,
    turn: usize,
}

impl Board {
    fn run(&mut self, turns: usize) {
        for _ in 0..turns {
            self.tick();
        }
    }

    fn tick(&mut self) {
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                let coord = Coordinate::new(x, y);

                if self[coord] != Tile::Occupied {
                    continue;
                }

                if !self.tile_has_neighbors(coord) {
                    continue;
                }

                let Some(direction) = Direction::MEMBERS
                    .into_iter()
                    .cycle()
                    .skip(self.turn % 4)
                    .take(4)
                    .find(|d| self.check_direction(coord, *d))
                else {
                    continue;
                };

                let tile = self.index_mut(coord.transform(direction));
                match *tile {
                    Tile::Empty => *tile = Tile::Proposed(direction.reversed()),
                    Tile::Proposed(_) => *tile = Tile::Blocked,
                    Tile::Occupied => (),
                    Tile::Blocked => (),
                }
            }
        }

        let mut x = 0;
        let mut y = 0;
        while y < self.board.len() {
            while x < self.board[y].len() {
                let coord = Coordinate::new(x, y);
                let tile = self.index_mut(coord);

                match *tile {
                    Tile::Empty => (),
                    Tile::Proposed(direction) => {
                        *tile = Tile::Occupied;
                        self[coord.transform(direction)] = Tile::Empty;
                        self.maybe_expand(&mut x, &mut y);
                    }
                    Tile::Occupied => self.maybe_expand(&mut x, &mut y),
                    Tile::Blocked => {
                        *tile = Tile::Empty;
                    }
                }

                x += 1;
            }

            y += 1;
        }

        self.turn += 1;
    }

    fn maybe_expand(&mut self, x: &mut usize, y: &mut usize) {
        if *y == 0 {
            self.board
                .iter_mut()
                .for_each(|row| row.push_front(Tile::Empty));
            *y = 1;
        } else if *y == self.board.len() - 1 {
            self.board
                .iter_mut()
                .for_each(|row| row.push_back(Tile::Empty));
            *y = self.board.len() - 2;
        }

        let width = self.board[0].len();
        if *x == 0 {
            let mut v = VecDeque::with_capacity(width);
            v.resize(width, Tile::Empty);
            self.board.push_front(v);
            *x = 1;
        } else if *x == width - 1 {
            let mut v = VecDeque::with_capacity(width);
            v.resize(width, Tile::Empty);
            self.board.push_front(v);
            *x = width - 1;
        }
    }

    fn tile_is_border(&self, coord: Coordinate) -> Option<[Option<Direction>; 2]> {
        match coord {
            Coordinate { x: 0, y: 0 } => Some([Some(Direction::North), Some(Direction::West)]),
            Coordinate { x: 0, y } if y == self.board.len() - 1 => {
                Some([Some(Direction::West), Some(Direction::South)])
            }
            Coordinate { x: 0, .. } => Some([Some(Direction::West), None]),
            Coordinate { x, y: 0 } if x == self.board[0].len() - 1 => {
                Some([Some(Direction::West), Some(Direction::South)])
            }

            _ => unimplemented!(),
        }
    }

    fn check_direction(&self, coord: Coordinate, direction: Direction) -> bool {
        !coord
            .get_neighbors(direction)
            .into_iter()
            .any(|c| self[c] == Tile::Occupied)
    }

    fn tile_has_neighbors(&self, coord: Coordinate) -> bool {
        coord
            .get_all_neighbors()
            .into_iter()
            .any(|c| self[c] == Tile::Occupied)
    }
}

impl Index<Coordinate> for Board {
    type Output = Tile;

    fn index(&self, Coordinate { x, y }: Coordinate) -> &Self::Output {
        self.board.index(y).index(x)
    }
}

impl IndexMut<Coordinate> for Board {
    fn index_mut(&mut self, Coordinate { x, y }: Coordinate) -> &mut Self::Output {
        self.board.index_mut(y).index_mut(x)
    }
}
