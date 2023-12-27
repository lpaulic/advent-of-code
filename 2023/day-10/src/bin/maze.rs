use std::fs::File;
use std::io::prelude::*;

extern crate day_10;
use day_10::PipeMaze;

fn main() -> std::io::Result<()> {
    let mut maze_layout_input_file =
        File::open("/home/lpaulic/Documents/github/advent-of-code/2023/day-10/src/bin/data/input")?;
    let mut maze_layout_input = String::new();

    maze_layout_input_file.read_to_string(&mut maze_layout_input)?;
    let pipe_maze = PipeMaze::parse(&maze_layout_input);
    println!(
        "Number of steps from start to furthest part of the maze: {}",
        pipe_maze.max_len_from_start()
    );

    println!(
        "Number of tiles inside the pipe loop: {}",
        pipe_maze.number_of_tiles_in_pipe()
    );

    Ok(())
}
