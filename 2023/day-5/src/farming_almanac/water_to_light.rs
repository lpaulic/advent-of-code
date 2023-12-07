use super::utils;

#[derive(Debug)]
pub struct WaterToLightMap {
    map: Vec<WaterToLight>,
}

#[derive(Debug)]
struct WaterToLight {
    water_range_start: u64,
    light_range_start: u64,
    range_size: u64,
}

impl WaterToLightMap {
    pub fn extract_water_to_light_map_from_almanac(almanac: &str) -> WaterToLightMap {
        const WATER_TO_LIGHT_KEY_STRING: &str = "water-to-light map:";
        WaterToLightMap {
            map: utils::extract_map_from_almanac(almanac, WATER_TO_LIGHT_KEY_STRING)
                .iter()
                .map(|line| {
                    let mut mapping = line.split_whitespace();
                    WaterToLight {
                        light_range_start: mapping.next().unwrap().parse::<u64>().unwrap(),
                        water_range_start: mapping.next().unwrap().parse::<u64>().unwrap(),
                        range_size: mapping.next().unwrap().parse::<u64>().unwrap(),
                    }
                })
                .collect(),
        }
    }

    pub fn water_to_light(&self, water: u64) -> u64 {
        match self.map.iter().find(|mapping| {
            water >= mapping.water_range_start
                && water < mapping.water_range_start + mapping.range_size
        }) {
            Some(mapping) => mapping.light_range_start + (water - mapping.water_range_start),
            None => water,
        }
    }
}
