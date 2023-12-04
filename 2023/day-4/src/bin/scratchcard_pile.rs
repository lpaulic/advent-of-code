use std::fs::File;
use std::io::prelude::*;

extern crate day_4;
use day_4::Scratchcards;

fn main() -> std::io::Result<()> {
    let mut scratchcards_input_file =
        File::open("/home/lpaulic/Documents/github/advent-of-code/2023/day-4/src/bin/data/input")?;
    let mut scratchcards_input = String::new();

    scratchcards_input_file.read_to_string(&mut scratchcards_input)?;
    let scratchcards = Scratchcards::parse(&scratchcards_input);
    println!("Total scratchcard points: {}", scratchcards.points_sum());
    println!(
        "Number of won scratch cards: {}",
        scratchcards.won_cards_count()
    );

    Ok(())
}
