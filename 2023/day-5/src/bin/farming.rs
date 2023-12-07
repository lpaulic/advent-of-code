use std::fs::File;
use std::io::prelude::*;

extern crate day_5;
use day_5::FarmingAlmanac;

fn main() -> std::io::Result<()> {
    let mut farming_input_file =
        File::open("/home/lpaulic/Documents/github/advent-of-code/2023/day-5/src/bin/data/input")?;
    let mut farming_input = String::new();

    farming_input_file.read_to_string(&mut farming_input)?;
    let farming = FarmingAlmanac::parse(&farming_input);
    println!("Lowest location number: {}", farming.get_min_location());

    Ok(())
}
