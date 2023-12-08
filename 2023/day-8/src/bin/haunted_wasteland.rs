use std::fs::File;
use std::io::prelude::*;

extern crate day_8;
use day_8::DesertMap;

fn main() -> std::io::Result<()> {
    let mut desert_map_input_file =
        File::open("/home/lpaulic/Documents/github/advent-of-code/2023/day-8/src/bin/data/input")?;
    let mut desert_map_input = String::new();

    desert_map_input_file.read_to_string(&mut desert_map_input)?;
    let desert_map = DesertMap::parse(&desert_map_input);
    println!(
        "Steps from 'AAA' to 'ZZZ': {}",
        desert_map.number_of_steps("AAA", "ZZZ")
    );

    println!(
        "Steps from '??A' to '??Z': {}",
        desert_map.number_of_ghost_steps('A', 'Z')
    );

    Ok(())
}
