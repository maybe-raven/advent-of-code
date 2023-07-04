mod blizzard_basin;
mod calorie_counting;
mod camp_cleanup;
mod full_of_hot_air;
mod monkey_map;
mod rock_paper_scissor;
mod rucksack_reorganization;
mod supply_stacks;
mod tuning_trouble;
mod unstable_difusion;

fn main() -> Result<(), String> {
    tuning_trouble::main()?;

    Ok(())
}
