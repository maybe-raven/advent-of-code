mod blizzard_basin;
mod calorie_counting;
mod full_of_hot_air;
mod monkey_map;
mod rock_paper_scissor;
mod rucksack_reorganization;
mod unstable_difusion;

fn main() -> Result<(), String> {
    rucksack_reorganization::main()?;

    Ok(())
}
