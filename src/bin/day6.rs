use std::{io, ops::Deref};

fn solutionate<S: Deref<Target = str>, I: IntoIterator<Item = io::Result<S>>>(
    input: I,
) -> Result<usize, String> {
    fn aux(line: &str) -> Option<usize> {
        let (_, line) = line.split_once(':')?;
        line.split_whitespace().collect::<String>().parse().ok()
    }

    let mut input_iter = input.into_iter();

    let time_str = &*input_iter
        .next()
        .ok_or("Missing input for time.")?
        .map_err(|e| e.to_string())?;
    let time =
        aux(time_str).ok_or_else(|| format!("Failed to parse input for time: {time_str}"))?;

    let distance_str = &*input_iter
        .next()
        .ok_or("Missing input for distance.")?
        .map_err(|e| e.to_string())?;
    let distance = aux(distance_str)
        .ok_or_else(|| format!("Failed to parse input for time: {distance_str}"))?;

    let answer = (1..time - 1)
        .filter(|charging_time| charging_time * (time - charging_time) > distance)
        .count();

    Ok(answer)
}

fn main() -> Result<(), String> {
    let answer = solutionate(io::stdin().lines())?;
    println!("{}", answer);
    Ok(())
}
