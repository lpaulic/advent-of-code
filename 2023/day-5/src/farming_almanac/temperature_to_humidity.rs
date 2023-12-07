use super::utils;

#[derive(Debug)]
pub struct TemperatureToHumidityMap {
    map: Vec<TemperatureToHumidity>,
}

#[derive(Debug)]
struct TemperatureToHumidity {
    temperature_range_start: u64,
    humidity_range_start: u64,
    range_size: u64,
}

impl TemperatureToHumidityMap {
    pub fn extract_temperature_to_humidity_map_from_almanac(
        almanac: &str,
    ) -> TemperatureToHumidityMap {
        const TEMPERATURE_TO_HUMIDITY_KEY_STRING: &str = "temperature-to-humidity map:";
        TemperatureToHumidityMap {
            map: utils::extract_map_from_almanac(almanac, TEMPERATURE_TO_HUMIDITY_KEY_STRING)
                .iter()
                .map(|line| {
                    let mut mapping = line.split_whitespace();
                    TemperatureToHumidity {
                        humidity_range_start: mapping.next().unwrap().parse::<u64>().unwrap(),
                        temperature_range_start: mapping.next().unwrap().parse::<u64>().unwrap(),
                        range_size: mapping.next().unwrap().parse::<u64>().unwrap(),
                    }
                })
                .collect(),
        }
    }

    pub fn temperature_to_humidity(&self, temperature: u64) -> u64 {
        match self.map.iter().find(|mapping| {
            temperature >= mapping.temperature_range_start
                && temperature < mapping.temperature_range_start + mapping.range_size
        }) {
            Some(mapping) => {
                mapping.humidity_range_start + (temperature - mapping.temperature_range_start)
            }
            None => temperature,
        }
    }
}
