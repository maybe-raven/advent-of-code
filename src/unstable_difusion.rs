//! Day 23: Unstable Diffusion
//! https://adventofcode.com/2022/day/23

#![allow(unused, dead_code)]

use std::{
    collections::VecDeque,
    fmt::{Display, Write},
    fs,
    iter::once,
    ops::{Index, IndexMut},
    str::FromStr,
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

    fn translate(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self::new(self.x, self.y - 1),
            Direction::South => Self::new(self.x, self.y + 1),
            Direction::West => Self::new(self.x - 1, self.y),
            Direction::East => Self::new(self.x + 1, self.y),
        }
    }

    fn get_neighbors(&self) -> [Self; 8] {
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

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Occupied,
    Proposed(Direction),
    Blocked,
}

impl Tile {
    fn is_occupied(self) -> bool {
        matches!(self, Tile::Occupied)
    }

    fn is_empty(self) -> bool {
        matches!(self, Tile::Empty)
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Occupied),
            _ => Err(()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(*self))
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Occupied => '#',
            Tile::Proposed(_) => '?',
            Tile::Blocked => 'x',
        }
    }
}

#[derive(Debug)]
struct Board {
    board: VecDeque<VecDeque<Tile>>,
    turn: usize,
}

impl Board {
    fn iterate(&mut self, turns: usize) {
        for _ in 0..turns {
            self.tick();
        }
    }

    fn run(&mut self) {
        while self.tick() {}
    }

    fn tick(&mut self) -> bool {
        let width = self.width();
        for y in 1..self.height() - 1 {
            for x in 1..width - 1 {
                let coord = Coordinate::new(x, y);

                if self[coord] != Tile::Occupied {
                    continue;
                }

                let availabilities = self.get_movement_availabilities(&coord);
                if availabilities == [true, true, true, true] {
                    continue;
                }

                let Some((direction, _)) = Direction::MEMBERS
                    .into_iter()
                    .zip(availabilities)
                    .cycle()
                    .skip(self.turn % 4)
                    .take(4)
                    .find(|x| x.1)
                else {
                    continue;
                };

                let target_coord = coord.translate(direction);
                let tile = self.index_mut(target_coord);
                match *tile {
                    Tile::Empty => *tile = Tile::Proposed(direction.reversed()),
                    Tile::Proposed(_) => *tile = Tile::Blocked,
                    Tile::Occupied => (),
                    Tile::Blocked => (),
                }
            }
        }

        let mut should_continue = false;
        let mut y = 0;
        while y < self.height() {
            let mut x = 0;
            while x < self.width() {
                let coord = Coordinate::new(x, y);
                let tile = self.index_mut(coord);

                match *tile {
                    Tile::Empty => (),
                    Tile::Proposed(direction) => {
                        should_continue = true;
                        *tile = Tile::Occupied;
                        self[coord.translate(direction)] = Tile::Empty;
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

        should_continue
    }

    fn get_movement_availabilities(&self, coord: &Coordinate) -> [bool; 4] {
        let [nw, n, ne, w, e, sw, s, se] = coord.get_neighbors().map(|x| !self[x].is_occupied());
        [nw && n && ne, sw && s && se, nw && w && sw, ne && e && se]
    }

    fn maybe_expand(&mut self, x: &mut usize, y: &mut usize) {
        if *x == 0 {
            self.board
                .iter_mut()
                .for_each(|row| row.push_front(Tile::Empty));
            *x = 1;
        } else if *x == self.width() - 1 {
            self.board
                .iter_mut()
                .for_each(|row| row.push_back(Tile::Empty));
        }

        let width = self.width();
        if *y == 0 {
            let mut v = VecDeque::with_capacity(width);
            v.resize(width, Tile::Empty);
            self.board.push_front(v);
            *y = 1;
        } else if *y == self.height() - 1 {
            let mut v = VecDeque::with_capacity(width);
            v.resize(width, Tile::Empty);
            self.board.push_back(v);
        }
    }

    fn width(&self) -> usize {
        self.board.front().map(|x| x.len()).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.board.len()
    }

    fn count_empty_tiles(&self) -> usize {
        let left_padding = self.count_left_padding();
        let right_padding = self.count_right_padding();
        let top_padding = self.count_top_padding();
        let bottom_padding = self.count_bottom_padding();

        self.board
            .iter()
            .skip(top_padding)
            .take(self.height() - top_padding - bottom_padding)
            .map(|row| {
                row.iter()
                    .skip(left_padding)
                    .take(self.width() - left_padding - right_padding)
                    .filter(|&&x| x.is_empty())
                    .count()
            })
            .sum()
    }

    fn count_bottom_padding(&self) -> usize {
        self.board
            .iter()
            .rev()
            .position(|row| row.iter().copied().any(Tile::is_occupied))
            .unwrap_or(0)
    }

    fn count_top_padding(&self) -> usize {
        self.board
            .iter()
            .position(|row| row.iter().copied().any(Tile::is_occupied))
            .unwrap_or(0)
    }

    fn count_left_padding(&self) -> usize {
        self.board
            .iter()
            .filter_map(|row| row.iter().copied().position(Tile::is_occupied))
            .min()
            .unwrap()
    }

    fn count_right_padding(&self) -> usize {
        self.board
            .iter()
            .filter_map(|row| row.iter().rev().copied().position(Tile::is_occupied))
            .min()
            .unwrap()
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

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let input_width = lines.next().ok_or(())?.len();
        let input_height = lines.count() + 1;

        let iter = s.lines().map(|line| {
            if line.len() != input_width {
                return Err(());
            }

            once('.')
                .chain(line.chars())
                .chain(once('.'))
                .map(Tile::try_from)
                .collect()
        });

        let board_width = input_width + 2;
        let mut padding = VecDeque::with_capacity(board_width);
        padding.resize(board_width, Tile::Empty);
        let board: VecDeque<VecDeque<Tile>> = once(Ok(padding.clone()))
            .chain(iter)
            .chain(once(Ok(padding)))
            .collect::<Result<_, Self::Err>>()?;

        Ok(Self { board, turn: 0 })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Turn: {}", self.turn)?;

        let s: String = self
            .board
            .iter()
            .flat_map(|row| row.iter().copied().map(char::from).chain(once('\n')))
            .collect();
        f.write_str(&s)
    }
}

const INPUT_FILENAME: &str = "input/unstable_difusion.txt";

pub fn main() -> Result<(), String> {
    let input = fs::read_to_string(INPUT_FILENAME).map_err(|e| e.to_string())?;
    let mut board: Board = input.parse().map_err(|_| "Parse error.")?;
    board.run();
    println!("{}", board);
    println!("{}", board.count_empty_tiles());
    println!("{}", board.turn);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_0: &str = "##
#.
..
##";

    const INPUT_1: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn test_movement_availabilities() {
        let mut board: Board = INPUT_0.parse().unwrap();
        // println!("{}", board);
        assert_eq!(
            [true, false, true, false],
            board.get_movement_availabilities(&Coordinate::new(1, 1))
        );
        assert_eq!(
            [true, false, false, true],
            board.get_movement_availabilities(&Coordinate::new(2, 1))
        );
        assert_eq!(
            [true, true, true, false],
            board.get_movement_availabilities(&Coordinate::new(1, 4))
        );
    }

    #[test]
    fn test_case_0() {
        let mut board: Board = INPUT_0.parse().unwrap();
        println!("{}", board);
        board.tick();
        println!("{}", board);
        board.tick();
        println!("{}", board);
        board.tick();
        println!("{}", board);
        board.tick();
        println!("{}", board);
    }

    #[test]
    fn test_case_1() {
        let mut board: Board = INPUT_1.parse().unwrap();
        println!("{}", board);
        for _ in 0..10 {
            board.tick();
            println!("{}", board);
        }
    }

    #[test]
    fn test_count_0() {
        let mut board: Board = INPUT_0.parse().unwrap();
        board.iterate(3);
        println!("{}", board);
        assert_eq!(1, board.count_left_padding());
        assert_eq!(1, board.count_right_padding());
        assert_eq!(25, board.count_empty_tiles());
    }

    #[test]
    fn test_count_1() {
        let mut board: Board = INPUT_1.parse().unwrap();
        board.iterate(10);
        assert_eq!(110, board.count_empty_tiles());
    }
}
