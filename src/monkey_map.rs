//! Day 22: Monkey Map
//! https://adventofcode.com/2022/day/22

#![allow(dead_code)]

use std::{fs::read_to_string, iter::once, num::NonZeroUsize, ops::Deref, str::FromStr};

trait IsNoneOr<T>: Copy {
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T: Copy> IsNoneOr<T> for Option<T> {
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Some(x) => f(x),
            None => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rotation {
    Clockwise,
    Counter,
}

impl TryFrom<char> for Rotation {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Counter),
            'R' => Ok(Self::Clockwise),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    Go(NonZeroUsize),
    Turn(Rotation),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(rotation) = s.chars().next().ok_or(())?.try_into() {
            Ok(Self::Turn(rotation))
        } else if let Ok(n) = s.parse() {
            Ok(Self::Go(n))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
struct CommandList(Vec<Command>);

impl FromStr for CommandList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut list = Vec::with_capacity(s.len() / 2);

        let mut start = 0;

        for (i, ch) in s.chars().enumerate() {
            if let Ok(rotation) = Rotation::try_from(ch) {
                let n = s[start..i].parse().map_err(|_| ())?;
                list.push(Command::Go(n));
                list.push(Command::Turn(rotation));

                start = i + 1;
            }
        }

        if start < s.len() {
            let n = s[start..].parse().map_err(|_| ())?;
            list.push(Command::Go(n));
        }

        Ok(CommandList(list))
    }
}

impl Deref for CommandList {
    type Target = [Command];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug, Clone, Copy)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    fn rotate(self, rotation: Rotation) -> Self {
        match (self, rotation) {
            (Facing::Up, Rotation::Clockwise) | (Facing::Down, Rotation::Counter) => Self::Right,
            (Facing::Up, Rotation::Counter) | (Facing::Down, Rotation::Clockwise) => Self::Left,
            (Facing::Left, Rotation::Clockwise) | (Facing::Right, Rotation::Counter) => Self::Up,
            (Facing::Left, Rotation::Counter) | (Facing::Right, Rotation::Clockwise) => Self::Down,
        }
    }
}

impl From<Facing> for usize {
    fn from(value: Facing) -> Self {
        match value {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Ground,
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ground),
            '#' => Ok(Self::Wall),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

struct Player {
    position: Coordinate,
    facing: Facing,
}

impl Player {
    fn new(x: usize) -> Self {
        Self {
            position: Coordinate { x, y: 0 },
            facing: Facing::Right,
        }
    }
}

struct Game {
    player: Player,
    board: Vec<Vec<Option<Tile>>>,
}

impl Game {
    fn width(&self) -> usize {
        self.board.first().map(|row| row.len()).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.board.len()
    }

    fn execute(&mut self, command: Command) {
        fn last_index_before_wall(mut iter: impl Iterator<Item = (usize, Tile)>) -> Option<usize> {
            let mut previous_index = None;
            while let Some((i, tile)) = iter.next() {
                if matches!(tile, Tile::Wall) {
                    break;
                }

                previous_index = Some(i);
            }

            previous_index
        }

        match command {
            Command::Go(n) => match self.player.facing {
                Facing::Up => {
                    let iter = self
                        .board
                        .iter()
                        .enumerate()
                        .rev()
                        .cycle()
                        .skip(self.height() - self.player.position.y)
                        .filter_map(|(y, row)| {
                            if let Some(x) = row.get(self.player.position.x).copied().flatten() {
                                Some((y, x))
                            } else {
                                None
                            }
                        })
                        .take(n.into());

                    if let Some(y) = last_index_before_wall(iter) {
                        self.player.position.y = y;
                    }
                }
                Facing::Down => {
                    let iter = self
                        .board
                        .iter()
                        .enumerate()
                        .cycle()
                        .skip(self.player.position.y)
                        .filter_map(|(y, row)| {
                            if let Some(x) = row.get(self.player.position.x).copied().flatten() {
                                Some((y, x))
                            } else {
                                None
                            }
                        })
                        .take(n.into());

                    if let Some(y) = last_index_before_wall(iter) {
                        self.player.position.y = y;
                    }
                }
                Facing::Left => {
                    let iter = self.board[self.player.position.y]
                        .iter()
                        .enumerate()
                        .rev()
                        .cycle()
                        .skip(self.width() - self.player.position.x)
                        .filter_map(|(y, &tile)| {
                            if let Some(x) = tile {
                                Some((y, x))
                            } else {
                                None
                            }
                        })
                        .take(n.into());

                    if let Some(x) = last_index_before_wall(iter) {
                        self.player.position.x = x;
                    }
                }
                Facing::Right => {
                    let iter = self.board[self.player.position.y]
                        .iter()
                        .enumerate()
                        .cycle()
                        .skip(self.player.position.x)
                        .filter_map(|(y, &tile)| {
                            if let Some(x) = tile {
                                Some((y, x))
                            } else {
                                None
                            }
                        })
                        .take(n.into());

                    if let Some(x) = last_index_before_wall(iter) {
                        self.player.position.x = x;
                    }
                }
            },
            Command::Turn(rotation) => self.player.facing = self.player.facing.rotate(rotation),
        }
    }

    fn run(&mut self, commands: &[Command]) {
        for &command in commands {
            self.execute(command);
        }
    }

    fn get_answer(&self) -> usize {
        let Coordinate { x, y } = self.player.position;
        1000 * (y + 1) + 4 * (x + 1) + usize::from(self.player.facing)
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first_line = lines.next().ok_or(())?;
        let first_row: Vec<Option<Tile>> = first_line
            .chars()
            .map(|ch| Tile::try_from(ch).ok())
            .collect();
        let x = first_row.partition_point(|x| x.is_none());
        let width = first_row.len();
        let iter = lines.map(|line| {
            let mut row = line
                .chars()
                .map(|ch| Tile::try_from(ch).ok())
                .collect::<Vec<_>>();
            row.resize(width, None);
            row
        });
        let board = once(first_row).chain(iter).collect();

        Ok(Self {
            player: Player::new(x),
            board,
        })
    }
}

const BOARD_FILENAME: &str = "input/monkey_map_input_board.txt";
const COMMANDS_FILENAME: &str = "input/monkey_map_input_commands.txt";

pub fn main() -> Result<(), String> {
    let commands: CommandList = read_to_string(COMMANDS_FILENAME)
        .map_err(|e| e.to_string())?
        .trim()
        .parse()
        .map_err(|_| "error parsing commands")?;

    let mut game: Game = read_to_string(BOARD_FILENAME)
        .map_err(|e| e.to_string())?
        .parse()
        .map_err(|_| "error parsing commands")?;

    game.run(&commands);

    println!("{}", game.get_answer());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_list_from_str() {
        let expected: &[Command] = &[
            Command::Go(NonZeroUsize::new(10).unwrap()),
            Command::Turn(Rotation::Clockwise),
            Command::Go(NonZeroUsize::new(5).unwrap()),
            Command::Turn(Rotation::Counter),
            Command::Go(NonZeroUsize::new(5).unwrap()),
            Command::Turn(Rotation::Clockwise),
            Command::Go(NonZeroUsize::new(10).unwrap()),
            Command::Turn(Rotation::Counter),
            Command::Go(NonZeroUsize::new(4).unwrap()),
            Command::Turn(Rotation::Clockwise),
            Command::Go(NonZeroUsize::new(5).unwrap()),
            Command::Turn(Rotation::Counter),
            Command::Go(NonZeroUsize::new(5).unwrap()),
        ];

        let result: &[Command] = &"10R5L5R10L4R5L5".parse::<CommandList>().unwrap();

        assert_eq!(expected, result);
    }
}
