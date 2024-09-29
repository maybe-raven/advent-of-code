mod blizzard_basin;
mod calorie_counting;
mod camp_cleanup;
mod cathode_ray_tube;
mod full_of_hot_air;
mod monkey_in_the_middle;
mod monkey_map;
mod no_space_left_on_device;
mod no_space_left_on_device_arena;
mod rock_paper_scissor;
mod rope_bridge;
mod rucksack_reorganization;
mod supply_stacks;
mod treetop_tree_house;
mod tuning_trouble;
mod unstable_difusion;

fn main() -> Result<(), String> {
    cathode_ray_tube::main()?;

    Ok(())
}
