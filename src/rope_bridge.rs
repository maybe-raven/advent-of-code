#![allow(dead_code)]

use std::{
    fmt::{Display, Write},
    io,
    str::FromStr,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn translate_tail(self, position: TailPosition) -> Self {
        let Coordinate { x, y } = self;
        match position {
            TailPosition::UpLeft => Self { x: x - 1, y: y - 1 },
            TailPosition::Up => Self { x, y: y - 1 },
            TailPosition::UpRight => Self { x: x + 1, y: y - 1 },
            TailPosition::Left => Self { x: x - 1, y },
            TailPosition::Overlap => self,
            TailPosition::Right => Self { x: x + 1, y },
            TailPosition::DownLeft => Self { x: x - 1, y: y + 1 },
            TailPosition::Down => Self { x, y: y + 1 },
            TailPosition::DownRight => Self { x: x + 1, y: y + 1 },
        }
    }

    fn translate_head(self, direction: MovementDirection) -> Self {
        let Coordinate { x, y } = self;
        match direction {
            MovementDirection::Up => Self { x, y: y - 1 },
            MovementDirection::Down => Self { x, y: y + 1 },
            MovementDirection::Left => Self { x: x - 1, y },
            MovementDirection::Right => Self { x: x + 1, y },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TailPosition {
    UpLeft,
    Up,
    UpRight,
    Left,
    Overlap,
    Right,
    DownLeft,
    Down,
    DownRight,
}

impl TailPosition {
    fn moveify(self, direction: MovementDirection) -> Self {
        match (self, direction) {
            (TailPosition::UpLeft, MovementDirection::Right)
            | (TailPosition::UpLeft, MovementDirection::Up)
            | (TailPosition::Overlap, MovementDirection::Right)
            | (TailPosition::DownLeft, MovementDirection::Right)
            | (TailPosition::DownLeft, MovementDirection::Down)
            | (TailPosition::Left, MovementDirection::Right) => Self::Left,
            (TailPosition::UpLeft, MovementDirection::Down)
            | (TailPosition::UpLeft, MovementDirection::Left)
            | (TailPosition::Overlap, MovementDirection::Down)
            | (TailPosition::UpRight, MovementDirection::Down)
            | (TailPosition::UpRight, MovementDirection::Right)
            | (TailPosition::Up, MovementDirection::Down) => Self::Up,
            (TailPosition::UpRight, MovementDirection::Left)
            | (TailPosition::UpRight, MovementDirection::Up)
            | (TailPosition::Overlap, MovementDirection::Left)
            | (TailPosition::DownRight, MovementDirection::Down)
            | (TailPosition::DownRight, MovementDirection::Left)
            | (TailPosition::Right, MovementDirection::Left) => Self::Right,
            (TailPosition::DownLeft, MovementDirection::Up)
            | (TailPosition::DownLeft, MovementDirection::Left)
            | (TailPosition::Overlap, MovementDirection::Up)
            | (TailPosition::DownRight, MovementDirection::Up)
            | (TailPosition::DownRight, MovementDirection::Right)
            | (TailPosition::Down, MovementDirection::Up) => Self::Down,
            (TailPosition::Left, MovementDirection::Left)
            | (TailPosition::Right, MovementDirection::Right)
            | (TailPosition::Down, MovementDirection::Down)
            | (TailPosition::Up, MovementDirection::Up) => Self::Overlap,
            (TailPosition::Right, MovementDirection::Down)
            | (TailPosition::Up, MovementDirection::Left) => Self::UpRight,
            (TailPosition::Left, MovementDirection::Down)
            | (TailPosition::Up, MovementDirection::Right) => Self::UpLeft,
            (TailPosition::Down, MovementDirection::Right)
            | (TailPosition::Left, MovementDirection::Up) => Self::DownLeft,
            (TailPosition::Down, MovementDirection::Left)
            | (TailPosition::Right, MovementDirection::Up) => Self::DownRight,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Movement {
    direction: MovementDirection,
    distance: i32,
}

impl Movement {
    fn from_str_opt(s: &str) -> Option<Self> {
        let mut iter = s.chars();
        let direction = match iter.next()? {
            'R' => MovementDirection::Right,
            'L' => MovementDirection::Left,
            'D' => MovementDirection::Down,
            'U' => MovementDirection::Up,
            _ => return None,
        };

        if !iter.next()?.is_whitespace() {
            return None;
        }

        let distance = s[2..].parse().ok()?;

        if 0 < distance {
            Some(Self {
                direction,
                distance,
            })
        } else {
            None
        }
    }
}

impl FromStr for Movement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_opt(s).ok_or_else(|| format!("{} is not a valid movement", s))
    }
}

impl Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self.direction {
            MovementDirection::Up => 'U',
            MovementDirection::Down => 'D',
            MovementDirection::Left => 'L',
            MovementDirection::Right => 'R',
        })?;
        f.write_char(' ')?;
        f.write_str(self.distance.to_string().as_str())
    }
}

struct Simropelacrum {
    head_coordinate: Coordinate,
    tail_position: TailPosition,
    position_log: Vec<Coordinate>,
}

impl Simropelacrum {
    fn new() -> Self {
        let coord = Coordinate { x: 0, y: 0 };
        Self {
            head_coordinate: coord,
            tail_position: TailPosition::Overlap,
            position_log: vec![coord],
        }
    }

    fn moveify(
        &mut self,
        Movement {
            direction,
            distance,
        }: Movement,
    ) {
        for _ in 0..distance {
            self.head_coordinate = self.head_coordinate.translate_head(direction);
            self.tail_position = self.tail_position.moveify(direction);
            self.position_log
                .push(self.head_coordinate.translate_tail(self.tail_position));
        }
    }

    fn cleanup(&mut self) {
        self.position_log.sort_unstable();
        self.position_log.dedup();
    }

    fn answerify(&mut self) -> usize {
        self.cleanup();
        self.position_log.len()
    }
}

impl Display for Simropelacrum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert!(!self.position_log.is_empty());

        let Coordinate {
            x: head_x,
            y: head_y,
        } = self.head_coordinate;
        let mut min_x = head_x;
        let mut max_x = head_x;
        let mut min_y = head_y;
        let mut max_y = head_y;

        for &Coordinate { x, y } in &self.position_log {
            if x < min_x {
                min_x = x;
            } else if max_x < x {
                max_x = x;
            }

            if y < min_y {
                min_y = y;
            } else if max_y < y {
                max_y = y;
            }
        }

        let mut grid = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

        for &Coordinate { x, y } in &self.position_log {
            grid[(y - min_y) as usize][(x - min_x) as usize] = '#';
        }

        grid[(head_y - min_y) as usize][(head_x - min_x) as usize] = 'H';
        let Coordinate { x, y } = self.head_coordinate.translate_tail(self.tail_position);
        grid[(y - min_y) as usize][(x - min_x) as usize] = 'T';

        let s = grid
            .into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .collect::<Vec<_>>();

        f.write_str(s.join("\n").as_str())
    }
}

pub fn main() -> Result<(), String> {
    let mut sim = Simropelacrum::new();
    for input in io::stdin().lines() {
        let line = input.map_err(|e| e.to_string())?;
        let movement: Movement = line.parse()?;
        sim.moveify(movement);
    }

    println!("{}", sim.answerify());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_from_str() {
        assert_eq!(
            Ok(Movement {
                direction: MovementDirection::Up,
                distance: 11
            }),
            "U 11".parse()
        );
    }

    #[test]
    fn test_simropelacrum() {
        let mut sim = Simropelacrum::new();
        sim.moveify(Movement {
            direction: MovementDirection::Right,
            distance: 4,
        });

        assert_eq!(4, sim.answerify());
        assert!(sim.position_log.contains(&Coordinate { x: 0, y: 0 }));
        assert!(sim.position_log.contains(&Coordinate { x: 1, y: 0 }));
        assert!(sim.position_log.contains(&Coordinate { x: 2, y: 0 }));
        assert!(sim.position_log.contains(&Coordinate { x: 3, y: 0 }));

        sim.moveify(Movement {
            direction: MovementDirection::Up,
            distance: 4,
        });

        assert_eq!(7, sim.answerify());
        assert!(sim.position_log.contains(&Coordinate { x: 0, y: 0 }));
        assert!(sim.position_log.contains(&Coordinate { x: 1, y: 0 }));
        assert!(sim.position_log.contains(&Coordinate { x: 2, y: 0 }));
        assert!(sim.position_log.contains(&Coordinate { x: 3, y: 0 }));
        assert!(sim.position_log.contains(&Coordinate { x: 4, y: -1 }));
        assert!(sim.position_log.contains(&Coordinate { x: 4, y: -2 }));
        assert!(sim.position_log.contains(&Coordinate { x: 4, y: -3 }));
    }

    #[test]
    fn test_solution() {
        const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        let movements: Vec<Movement> = INPUT
            .lines()
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .unwrap();

        let mut sim = Simropelacrum::new();
        for m in movements {
            sim.moveify(m);
            println!("{}", m);
            println!("{}", sim);
            println!("");
        }

        assert_eq!(13, sim.answerify());
    }
}
