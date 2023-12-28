#[derive(Debug, Clone, Copy)]
struct Galaxy {
    id: u64,
    position_x: u64,
    position_y: u64,
}

pub struct SpaceImage {
    galaxy_pairs: Vec<(Galaxy, Galaxy)>,
}

impl SpaceImage {
    pub fn parse(image: &str) -> Self {
        let mut galaxies = SpaceImage::parse_galaxies(image);
        let empty_space_columns = SpaceImage::parse_empty_space_columns(image);
        let empty_space_rows = SpaceImage::parse_empty_space_rows(image);

        SpaceImage::expand_space(&mut galaxies, &empty_space_columns, &empty_space_rows);

        SpaceImage {
            galaxy_pairs: SpaceImage::extract_galaxy_pairs(&galaxies),
        }
    }

    fn parse_galaxies(image: &str) -> Vec<Galaxy> {
        let mut galaxies: Vec<Galaxy> = Vec::new();
        let mut galaxy_id = 1_u64;
        image
            .split_whitespace()
            .enumerate()
            .for_each(|(row_index, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .for_each(|(column_index, c)| {
                        if c == '#' {
                            galaxies.push(Galaxy {
                                id: galaxy_id,
                                position_x: row_index as u64,
                                position_y: column_index as u64,
                            });

                            galaxy_id += 1;
                        }
                    });
            });

        galaxies
    }

    fn parse_empty_space_columns(image: &str) -> Vec<u64> {
        let mut empty_space_columns: Vec<u64> = Vec::new();

        let image_column_count = image
            .split_whitespace()
            .next()
            .unwrap()
            .trim()
            .chars()
            .count();

        for column_index in 0..image_column_count {
            let mut is_empty = true;
            image.split_whitespace().for_each(|line| {
                if let Some(c) = line.chars().nth(column_index) {
                    if c != '.' {
                        is_empty = false;
                    }
                }
            });

            if is_empty {
                empty_space_columns.push(column_index as u64)
            }
        }

        empty_space_columns
    }

    fn parse_empty_space_rows(image: &str) -> Vec<u64> {
        let mut empty_space_rows: Vec<u64> = Vec::new();

        image
            .split_whitespace()
            .enumerate()
            .for_each(|(row_index, line)| {
                let mut is_empty = true;
                line.trim().chars().for_each(|c| {
                    if c != '.' {
                        is_empty = false;
                    }
                });

                if is_empty {
                    empty_space_rows.push(row_index as u64)
                }
            });

        empty_space_rows
    }

    fn expand_space(
        galaxies: &mut [Galaxy],
        empty_space_columns: &[u64],
        empty_space_rows: &[u64],
    ) {
        galaxies.iter_mut().for_each(|galaxy| {
            const GALAXY_AGE_CONSTANT: u64 = 1000000;

            galaxy.position_x += GALAXY_AGE_CONSTANT
                * empty_space_rows
                    .iter()
                    .filter(|column| **column < galaxy.position_x)
                    .count() as u64;

            galaxy.position_y += GALAXY_AGE_CONSTANT
                * empty_space_columns
                    .iter()
                    .filter(|column| **column < galaxy.position_y)
                    .count() as u64;
        });
    }

    fn extract_galaxy_pairs(galaxies: &[Galaxy]) -> Vec<(Galaxy, Galaxy)> {
        let mut galaxy_pairs: Vec<(Galaxy, Galaxy)> = Vec::new();
        for galaxy in galaxies {
            let galaxy_index = galaxies.iter().position(|g| g.id == galaxy.id).unwrap();
            galaxies
                .iter()
                .skip(galaxy_index + 1)
                .for_each(|g| galaxy_pairs.push((*galaxy, *g)));
        }

        galaxy_pairs
    }

    pub fn get_shortest_path_between_galaxy_pairs_sum(&self) -> u64 {
        self.galaxy_pairs
            .iter()
            .map(|galaxy_pair| SpaceImage::shortest_path(&galaxy_pair.0, &galaxy_pair.1))
            .sum()
    }

    fn shortest_path(galaxy1: &Galaxy, galaxy2: &Galaxy) -> u64 {
        galaxy1.position_x.abs_diff(galaxy2.position_x)
            + galaxy1.position_y.abs_diff(galaxy2.position_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_shortest_path_sum(expected_result: u64, space_image: &str) {
        assert_eq!(
            expected_result,
            SpaceImage::parse(space_image).get_shortest_path_between_galaxy_pairs_sum()
        );
    }

    #[test]
    fn shortest_path_between_two_galaxies_on_one_line() {
        assert_shortest_path_sum(1, "##");
        assert_shortest_path_sum(1000002, ".#.#.");
    }

    #[test]
    fn shortest_path_between_two_galaxies_on_multiple_lines() {
        assert_shortest_path_sum(1, "#\n#\n");
        assert_shortest_path_sum(2, "\n.#.\n..#\n");
        assert_shortest_path_sum(1000002, "\n.#.\n...\n.#.\n");
    }

    #[test]
    fn shortest_path_between_three_galaxies_on_one_line() {
        assert_shortest_path_sum(4000008, "#.#.#");
        assert_shortest_path_sum(12000016, ".#..#....#");
    }

    #[test]
    fn shortest_path_between_three_galaxies_on_multiple_lines() {
        assert_shortest_path_sum(8, "#..\n.#.\n..#");
        assert_shortest_path_sum(4, ".#.\n.#.\n.#.\n");
    }

    #[test]
    fn shortest_path_between_nine_galaxies_on_multiple_lines() {
        assert_shortest_path_sum(82000292, "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....");
    }
}
