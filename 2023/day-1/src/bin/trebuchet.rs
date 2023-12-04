use std::fs::File;
use std::io::prelude::*;

extern crate day_1;
use day_1::Calibration;

fn main() -> std::io::Result<()> {
    let mut calibration_input_file =
        File::open("/home/lpaulic/Documents/github/advent-of-code/2023/day-1/src/bin/data/input")?;
    let mut calibration_input = String::new();

    calibration_input_file.read_to_string(&mut calibration_input)?;
    let calibration = Calibration::parse(&calibration_input);
    println!("Sum of calibration values: {}", calibration.sum());

    Ok(())
}
