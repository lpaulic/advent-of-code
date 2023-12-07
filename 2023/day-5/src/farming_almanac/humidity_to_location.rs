use super::utils;

#[derive(Debug)]
pub struct HumidityToLocationMap {
    map: Vec<HumidityToLocation>,
}

#[derive(Debug)]
struct HumidityToLocation {
    humidity_range_start: u64,
    location_range_start: u64,
    range_size: u64,
}

impl HumidityToLocationMap {
    pub fn extract_humidity_to_location_map_from_almanac(almanac: &str) -> HumidityToLocationMap {
        const HUMIDITY_TO_LOCATION_KEY_STRING: &str = "humidity-to-location map:";
        HumidityToLocationMap {
            map: utils::extract_map_from_almanac(almanac, HUMIDITY_TO_LOCATION_KEY_STRING)
                .iter()
                .map(|line| {
                    let mut mapping = line.split_whitespace();
                    HumidityToLocation {
                        location_range_start: mapping.next().unwrap().parse::<u64>().unwrap(),
                        humidity_range_start: mapping.next().unwrap().parse::<u64>().unwrap(),
                        range_size: mapping.next().unwrap().parse::<u64>().unwrap(),
                    }
                })
                .collect(),
        }
    }

    pub fn humidity_to_location(&self, humidity: u64) -> u64 {
        match self.map.iter().find(|mapping| {
            humidity >= mapping.humidity_range_start
                && humidity < mapping.humidity_range_start + mapping.range_size
        }) {
            Some(mapping) => {
                mapping.location_range_start + (humidity - mapping.humidity_range_start)
            }
            None => humidity,
        }
    }
}
