use super::utils;

#[derive(Debug)]
pub struct LightToTemperatureMap {
    map: Vec<LightToTemperature>,
}

#[derive(Debug)]
struct LightToTemperature {
    light_range_start: u64,
    temperature_range_start: u64,
    range_size: u64,
}

impl LightToTemperatureMap {
    pub fn extract_light_to_temperature_map_from_almanac(almanac: &str) -> LightToTemperatureMap {
        const LIGHT_TO_TEMPERATURE_KEY_STRING: &str = "light-to-temperature map:";
        LightToTemperatureMap {
            map: utils::extract_map_from_almanac(almanac, LIGHT_TO_TEMPERATURE_KEY_STRING)
                .iter()
                .map(|line| {
                    let mut mapping = line.split_whitespace();
                    LightToTemperature {
                        temperature_range_start: mapping.next().unwrap().parse::<u64>().unwrap(),
                        light_range_start: mapping.next().unwrap().parse::<u64>().unwrap(),
                        range_size: mapping.next().unwrap().parse::<u64>().unwrap(),
                    }
                })
                .collect(),
        }
    }

    pub fn light_to_temperature(&self, light: u64) -> u64 {
        match self.map.iter().find(|mapping| {
            light >= mapping.light_range_start
                && light < mapping.light_range_start + mapping.range_size
        }) {
            Some(mapping) => mapping.temperature_range_start + (light - mapping.light_range_start),
            None => light,
        }
    }
}
