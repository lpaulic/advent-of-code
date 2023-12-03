use std::fs::File;
use std::io::prelude::*;

extern crate day_three;
use day_three::EngineSchematic;

fn main() -> std::io::Result<()> {
    let mut schematic_input_file = File::open(
        "/home/lpaulic/Documents/github/advent-of-code/2023/day-three/src/bin/data/input",
    )?;
    let mut schematic_input = String::new();

    schematic_input_file.read_to_string(&mut schematic_input)?;
    let schematic = EngineSchematic::parse(&schematic_input);
    println!(
        "Sum of engine schematic part numbers: {}",
        schematic.part_number_sum()
    );

    println!(
        "Sum of engine schematic gear ratios: {}",
        schematic.gear_ratio_sum()
    );

    Ok(())
}
