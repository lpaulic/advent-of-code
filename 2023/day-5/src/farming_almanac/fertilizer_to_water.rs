use super::utils;

#[derive(Debug)]
pub struct FertilizerToWaterMap {
    map: Vec<FertilizerToWater>,
}

#[derive(Debug)]
struct FertilizerToWater {
    fertilizer_range_start: u64,
    water_range_start: u64,
    range_size: u64,
}

impl FertilizerToWaterMap {
    pub fn extract_fertilizer_to_water_map_from_almanac(almanac: &str) -> FertilizerToWaterMap {
        const FERTILIZER_TO_WATER_KEY_STRING: &str = "fertilizer-to-water map:";
        FertilizerToWaterMap {
            map: utils::extract_map_from_almanac(almanac, FERTILIZER_TO_WATER_KEY_STRING)
                .iter()
                .map(|line| {
                    let mut mapping = line.split_whitespace();
                    FertilizerToWater {
                        water_range_start: mapping.next().unwrap().parse::<u64>().unwrap(),
                        fertilizer_range_start: mapping.next().unwrap().parse::<u64>().unwrap(),
                        range_size: mapping.next().unwrap().parse::<u64>().unwrap(),
                    }
                })
                .collect(),
        }
    }

    pub fn fertilizer_to_water(&self, fertilizer: u64) -> u64 {
        match self.map.iter().find(|mapping| {
            fertilizer >= mapping.fertilizer_range_start
                && fertilizer < mapping.fertilizer_range_start + mapping.range_size
        }) {
            Some(mapping) => {
                mapping.water_range_start + (fertilizer - mapping.fertilizer_range_start)
            }
            None => fertilizer,
        }
    }
}
