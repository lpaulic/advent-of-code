use std::fs::File;
use std::io::prelude::*;

extern crate day_9;
use day_9::OasisPredictor;

fn main() -> std::io::Result<()> {
    let mut oasis_report_input_file =
        File::open("/home/lpaulic/Documents/github/advent-of-code/2023/day-9/src/bin/data/input")?;
    let mut oasis_report_input = String::new();

    oasis_report_input_file.read_to_string(&mut oasis_report_input)?;
    let oasis_predictor = OasisPredictor::parse(&oasis_report_input);
    println!(
        "Next value prediction sum: {}",
        oasis_predictor.next_value_predictions_sum()
    );

    println!(
        "Previous value prediction sum: {}",
        oasis_predictor.previous_value_predictions_sum()
    );

    Ok(())
}
