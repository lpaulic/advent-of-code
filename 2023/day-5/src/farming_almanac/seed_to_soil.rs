use super::utils;

#[derive(Debug)]
pub struct SeedToSoilMap {
    map: Vec<SeedToSoil>,
}

#[derive(Debug)]
struct SeedToSoil {
    seed_range_start: u64,
    soil_range_start: u64,
    range_size: u64,
}

impl SeedToSoilMap {
    pub fn extract_seed_to_soil_map_from_almanac(almanac: &str) -> SeedToSoilMap {
        const SEED_TO_SOIL_KEY_STRING: &str = "seed-to-soil map:";
        SeedToSoilMap {
            map: utils::extract_map_from_almanac(almanac, SEED_TO_SOIL_KEY_STRING)
                .iter()
                .map(|line| {
                    let mut mapping = line.split_whitespace();
                    SeedToSoil {
                        soil_range_start: mapping.next().unwrap().parse::<u64>().unwrap(),
                        seed_range_start: mapping.next().unwrap().parse::<u64>().unwrap(),
                        range_size: mapping.next().unwrap().parse::<u64>().unwrap(),
                    }
                })
                .collect(),
        }
    }

    pub fn seed_to_soil(&self, seed: u64) -> u64 {
        match self.map.iter().find(|mapping| {
            seed >= mapping.seed_range_start && seed < mapping.seed_range_start + mapping.range_size
        }) {
            Some(mapping) => mapping.soil_range_start + (seed - mapping.seed_range_start),
            None => seed,
        }
    }
}
