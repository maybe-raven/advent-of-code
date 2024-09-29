#![allow(dead_code)]

use std::io;

#[derive(Debug, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissor,
}

impl Play {
    fn score(self) -> usize {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissor => 3,
        }
    }

    fn score_against(self, other: Self) -> usize {
        self.score() + self.winnage_against(other).score()
    }

    fn winnage_against(self, other: Self) -> Winnage {
        match (self, other) {
            (Play::Rock, Play::Rock)
            | (Play::Paper, Play::Paper)
            | (Play::Scissor, Play::Scissor) => Winnage::Draw,
            (Play::Rock, Play::Paper)
            | (Play::Paper, Play::Scissor)
            | (Play::Scissor, Play::Rock) => Winnage::Loss,
            (Play::Rock, Play::Scissor)
            | (Play::Paper, Play::Rock)
            | (Play::Scissor, Play::Paper) => Winnage::Win,
        }
    }

    fn deduce_play(self, goal: Winnage) -> Self {
        match (self, goal) {
            (Play::Rock, Winnage::Win)
            | (Play::Scissor, Winnage::Loss)
            | (Play::Paper, Winnage::Draw) => Self::Paper,
            (Play::Rock, Winnage::Loss)
            | (Play::Paper, Winnage::Win)
            | (Play::Scissor, Winnage::Draw) => Self::Scissor,
            (Play::Scissor, Winnage::Win)
            | (Play::Paper, Winnage::Loss)
            | (Play::Rock, Winnage::Draw) => Self::Rock,
        }
    }
}

impl TryFrom<char> for Play {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissor),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Winnage {
    Win,
    Loss,
    Draw,
}

impl Winnage {
    fn score(self) -> usize {
        match self {
            Winnage::Win => 6,
            Winnage::Draw => 3,
            Winnage::Loss => 0,
        }
    }
}

impl TryFrom<char> for Winnage {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

pub fn main() -> Result<(), String> {
    let result = solutionate(io::stdin().lines())?;
    println!("{}", result);

    Ok(())
}

fn solutionate<I: Iterator<Item = io::Result<String>>>(iter: I) -> Result<usize, String> {
    iter.map(|x| {
        x.map_err(|e| e.to_string()).and_then(|line| {
            parse_round(line.as_str()).map_err(|_| format!("unable to parse line `{}`", line))
        })
    })
    .sum::<Result<usize, String>>()
}

fn parse_round(s: &str) -> Result<usize, ()> {
    let mut iter = s.chars();
    let opponent_play = Play::try_from(iter.next().ok_or(())?)?;
    iter.next();
    let goal = Winnage::try_from(iter.next().ok_or(())?)?;
    let my_play = opponent_play.deduce_play(goal);

    Ok(my_play.score_against(opponent_play))
}
