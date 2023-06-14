//! Day 24: Blizzard Basin
//! https://adventofcode.com/2022/day/24

use std::{cmp::min, ops::Index};

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
enum Blizzard {
    Up,
    Down,
    Left,
    Right,
}

impl Blizzard {
    fn update_coordinate<const Width: usize, const Height: usize>(
        self,
        Coordinate { x, y }: Coordinate<Width, Height>,
    ) -> Coordinate<Width, Height> {
        match self {
            Blizzard::Up => Coordinate::new_unchecked(x, y.checked_dec()),
            Blizzard::Down => Coordinate::new(x, y + 1),
            Blizzard::Left => Coordinate::new_unchecked(x.checked_dec(), y),
            Blizzard::Right => Coordinate::new(x + 1, y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Coordinate<const Width: usize, const Height: usize> {
    x: usize,
    y: usize,
}

impl<const Width: usize, const Height: usize> Coordinate<Width, Height> {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: min(x, Width - 1),
            y: min(y, Height - 1),
        }
    }

    fn new_unchecked(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Valley<const Width: usize, const Height: usize>([[Blizzard; Width]; Height]);

impl<const Width: usize, const Height: usize> Valley<Width, Height> {
    fn tick(&mut self) {
        todo!()
    }
}

impl<const Width: usize, const Height: usize> Index<Coordinate<Width, Height>>
    for Valley<Width, Height>
{
    type Output = Blizzard;

    fn index(&self, index: Coordinate<Width, Height>) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}
