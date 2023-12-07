use std::fs::File;
use std::io::prelude::*;

extern crate day_7;
use day_7::CamelCards;

fn main() -> std::io::Result<()> {
    let mut came_cards_input_file =
        File::open("/home/lpaulic/Documents/github/advent-of-code/2023/day-7/src/bin/data/input")?;
    let mut came_cards_input = String::new();

    came_cards_input_file.read_to_string(&mut came_cards_input)?;
    let camel_cards = CamelCards::new(&came_cards_input);
    println!("Total winings: {}", camel_cards.get_total_winnings());

    Ok(())
}
