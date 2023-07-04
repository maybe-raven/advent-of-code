mod blizzard_basin;
mod calorie_counting;
mod camp_cleanup;
mod full_of_hot_air;
mod monkey_map;
mod no_space_left_on_device;
mod rock_paper_scissor;
mod rucksack_reorganization;
mod supply_stacks;
mod tuning_trouble;
mod unstable_difusion;

fn main() -> Result<(), String> {
    no_space_left_on_device::main()?;

    Ok(())
}
