use std::fs::File;
use std::io::prelude::*;

extern crate day_6;
use day_6::RaceStatistics;

fn main() -> std::io::Result<()> {
    let mut race_statistics_input_file =
        File::open("/home/lpaulic/Documents/github/advent-of-code/2023/day-6/src/bin/data/input")?;
    let mut race_statistics_input = String::new();

    race_statistics_input_file.read_to_string(&mut race_statistics_input)?;
    let race_statistics_part1 = RaceStatistics::parse(&race_statistics_input, false);
    let race_statistics_part2 = RaceStatistics::parse(&race_statistics_input, true);
    println!(
        "Product of record break opportunities(with whitespace): {}",
        race_statistics_part1.get_race_record_break_product()
    );

    println!(
        "Product of record break opportunities(without whitespace): {}",
        race_statistics_part2.get_race_record_break_product()
    );

    Ok(())
}
