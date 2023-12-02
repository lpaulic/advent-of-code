use std::fs::File;
use std::io::prelude::*;

extern crate day_two;
use day_two::Game;

fn main() -> std::io::Result<()> {
    let available_balls = "12 red, 13 green, 14 blue";
    let mut cube_conundrum_input_file = File::open(
        "/home/lpaulic/Documents/github/advent-of-code/2023/day-two/src/bin/data/input",
    )?;
    let mut cube_conundrum_input = String::new();

    cube_conundrum_input_file.read_to_string(&mut cube_conundrum_input)?;
    let cube_conundrum_game = Game::new(available_balls);
    println!(
        "Sum of possible game ids: {}",
        cube_conundrum_game.possible_games_id_sum(&cube_conundrum_input)
    );

    println!(
        "Sum of game cube power: {}",
        cube_conundrum_game.game_power_sum(&cube_conundrum_input)
    );

    Ok(())
}
