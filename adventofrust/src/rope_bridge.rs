#![allow(dead_code)]

use std::{
    fmt::{Display, Write},
    io,
    ops::IndexMut,
    str::FromStr,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn translate(&mut self, direction: MovementDirection) {
        match direction {
            MovementDirection::Up => self.y -= 1,
            MovementDirection::Down => self.y += 1,
            MovementDirection::Left => self.x -= 1,
            MovementDirection::Right => self.x += 1,
        }
    }

    fn chase(&mut self, head: Coordinate) -> bool {
        let dx = head.x - self.x;
        let dy = head.y - self.y;

        if dx.abs() < 2 && dy.abs() < 2 {
            return false;
        }

        if dx == 0 {
            self.y += dy.signum();
        } else if dy == 0 {
            self.x += dx.signum();
        } else {
            self.x += dx.signum();
            self.y += dy.signum();
        }

        true
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

struct Simropelacrum<const N: usize> {
    knots: [Coordinate; N],
    position_log: Vec<Coordinate>,
}

impl<const N: usize> Simropelacrum<N> {
    const MAXI: usize = N - 1;

    fn new() -> Self {
        let coord = Coordinate { x: 0, y: 0 };
        Self {
            knots: [coord; N],
            position_log: vec![coord],
        }
    }

    fn moveify(&mut self, movement: Movement) {
        if N == 0 {
            return;
        }

        for _ in 0..movement.distance {
            self.knots[0].translate(movement.direction);
            for i in 1..self.knots.len() {
                let previous_knot = self.knots[i - 1];
                let current_knot = self.knots.index_mut(i);
                if !current_knot.chase(previous_knot) {
                    break;
                } else if i == Self::MAXI {
                    self.position_log.push(*current_knot);
                }
            }
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

impl<const N: usize> Display for Simropelacrum<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert!(!self.position_log.is_empty());

        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for &Coordinate { x, y } in &self.knots {
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

        for (i, &Coordinate { x, y }) in self.knots.iter().enumerate() {
            grid[(y - min_y) as usize][(x - min_x) as usize] =
                char::from_digit(i as u32, 36).unwrap_or('*');
        }

        let s = grid
            .into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .collect::<Vec<_>>();

        f.write_str(s.join("\n").as_str())
    }
}

pub fn main() -> Result<(), String> {
    let mut sim = Simropelacrum::<10>::new();
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
    fn test_solution() {
        const INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        let movements: Vec<Movement> = INPUT
            .lines()
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .unwrap();

        let mut sim = Simropelacrum::<10>::new();
        for m in movements {
            sim.moveify(m);
            println!("{}", m);
            println!("{}", sim);
            println!("");
        }

        assert_eq!(36, sim.answerify());
    }

    #[test]
    fn test_chase() {
        let mut coord = Coordinate { x: 0, y: 0 };
        coord.chase(Coordinate { x: 0, y: 0 });
        assert_eq!(Coordinate { x: 0, y: 0 }, coord);
        coord.chase(Coordinate { x: 1, y: 1 });
        assert_eq!(Coordinate { x: 0, y: 0 }, coord);
        coord.chase(Coordinate { x: 2, y: 0 });
        assert_eq!(Coordinate { x: 1, y: 0 }, coord);
        coord.chase(Coordinate { x: 3, y: 1 });
        assert_eq!(Coordinate { x: 2, y: 1 }, coord);
    }
}
