use super::utils;

#[derive(Debug)]
pub struct SoilToFertilizerMap {
    map: Vec<SoilToFertilizer>,
}

#[derive(Debug)]
struct SoilToFertilizer {
    soil_range_start: u64,
    fertilizer_range_start: u64,
    range_size: u64,
}

impl SoilToFertilizerMap {
    pub fn extract_soil_to_fertilizer_map_from_almanac(almanac: &str) -> SoilToFertilizerMap {
        const SOIL_TO_FERTILIZER_KEY_STRING: &str = "soil-to-fertilizer map:";
        SoilToFertilizerMap {
            map: utils::extract_map_from_almanac(almanac, SOIL_TO_FERTILIZER_KEY_STRING)
                .iter()
                .map(|line| {
                    let mut mapping = line.split_whitespace();
                    SoilToFertilizer {
                        fertilizer_range_start: mapping.next().unwrap().parse::<u64>().unwrap(),
                        soil_range_start: mapping.next().unwrap().parse::<u64>().unwrap(),
                        range_size: mapping.next().unwrap().parse::<u64>().unwrap(),
                    }
                })
                .collect(),
        }
    }

    pub fn soil_to_fertilizer(&self, soil: u64) -> u64 {
        match self.map.iter().find(|mapping| {
            soil >= mapping.soil_range_start && soil < mapping.soil_range_start + mapping.range_size
        }) {
            Some(mapping) => mapping.fertilizer_range_start + (soil - mapping.soil_range_start),
            None => soil,
        }
    }
}
