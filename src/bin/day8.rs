use std::{collections::HashMap, ops::Index};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        if c == b'L' {
            Ok(Self::Left)
        } else if c == b'R' {
            Ok(Self::Right)
        } else {
            Err(())
        }
    }
}

impl<T> Index<Direction> for (T, T) {
    type Output = T;

    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::Left => &self.0,
            Direction::Right => &self.1,
        }
    }
}

fn main() -> Result<(), String> {
    let mut input = std::io::stdin().lines();

    let line = input
        .next()
        .ok_or("Missing input".to_owned())?
        .map_err(|e| e.to_string())?;

    if !line.is_ascii() {
        return Err("Only ASCII string is supported.".to_owned());
    }

    let instructions = line
        .bytes()
        .map(Direction::try_from)
        .collect::<Result<Vec<Direction>, ()>>()
        .map_err(|_| format!("Failed to parse instruction on first line: {line}"))?;

    let map = input
        .filter_map(|input| {
            let line = match input {
                Ok(line) => line,
                Err(e) => return Some(Err(e.to_string())),
            };

            let (node_id, nexts) = line.trim().split_once('=')?;
            let (next_left, next_right) = nexts
                .trim_start()
                .strip_prefix('(')?
                .strip_suffix(')')?
                .split_once(',')?;

            Some(Ok((
                node_id.trim_end().to_owned(),
                (next_left.trim().to_owned(), next_right.trim().to_owned()),
            )))
        })
        .collect::<Result<HashMap<String, (String, String)>, String>>()?;

    let mut nodes: Vec<&String> = map.keys().filter(|s| s.ends_with('A')).collect();

    println!("Start: {:?}", &nodes);

    for (step, dir) in instructions.into_iter().cycle().enumerate() {
        if nodes.iter().all(|node| node.ends_with('Z')) {
            println!("{}", step);
            return Ok(());
        }
        print!("Step {step}: ");
        for node in nodes.iter_mut() {
            print!("{node} -> ");
            *node = &map
                .get(*node)
                .ok_or_else(|| format!("Requested node doesn't exist: {node}"))?[dir];
            print!("{node}; ");
        }
        println!();
    }

    unreachable!()
}
