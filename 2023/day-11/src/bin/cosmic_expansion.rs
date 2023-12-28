use std::fs::File;
use std::io::prelude::*;

extern crate day_11;
use day_11::SpaceImage;

fn main() -> std::io::Result<()> {
    let mut space_image_input_file =
        File::open("/home/lpaulic/Documents/github/advent-of-code/2023/day-11/src/bin/data/input")?;
    let mut space_image_input = String::new();

    space_image_input_file.read_to_string(&mut space_image_input)?;
    let space_image = SpaceImage::parse(&space_image_input);
    println!(
        "Sum of shortest paths between galaxy pairs: {}",
        space_image.get_shortest_path_between_galaxy_pairs_sum()
    );

    Ok(())
}
