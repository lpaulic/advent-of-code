mod fertilizer_to_water;
mod humidity_to_location;
mod light_to_temperature;
mod seed_to_soil;
mod soil_to_fertilizer;
mod temperature_to_humidity;
mod utils;
mod water_to_light;

use self::fertilizer_to_water::*;
use self::humidity_to_location::*;
use self::light_to_temperature::*;
use self::seed_to_soil::*;
use self::soil_to_fertilizer::*;
use self::temperature_to_humidity::*;
use self::water_to_light::*;

#[derive(Debug)]
pub struct FarmingAlmanac {
    seeds: Vec<u64>,
    seeds_to_soil_map: SeedToSoilMap,
    soil_to_fertilizer_map: SoilToFertilizerMap,
    fertilizer_to_water_map: FertilizerToWaterMap,
    water_to_light_map: WaterToLightMap,
    light_to_temperature_map: LightToTemperatureMap,
    temperature_to_humidity_map: TemperatureToHumidityMap,
    humidity_to_location_map: HumidityToLocationMap,
}

impl FarmingAlmanac {
    pub fn parse(almanac: &str) -> Self {
        FarmingAlmanac {
            seeds: FarmingAlmanac::extract_seeds_from_almanac(almanac),
            seeds_to_soil_map: SeedToSoilMap::extract_seed_to_soil_map_from_almanac(almanac),
            soil_to_fertilizer_map:
                SoilToFertilizerMap::extract_soil_to_fertilizer_map_from_almanac(almanac),
            fertilizer_to_water_map:
                FertilizerToWaterMap::extract_fertilizer_to_water_map_from_almanac(almanac),
            water_to_light_map: WaterToLightMap::extract_water_to_light_map_from_almanac(almanac),
            light_to_temperature_map:
                LightToTemperatureMap::extract_light_to_temperature_map_from_almanac(almanac),
            temperature_to_humidity_map:
                TemperatureToHumidityMap::extract_temperature_to_humidity_map_from_almanac(almanac),
            humidity_to_location_map:
                HumidityToLocationMap::extract_humidity_to_location_map_from_almanac(almanac),
        }
    }

    pub fn get_min_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| self.seeds_to_soil_map.seed_to_soil(*seed))
            .map(|soil| self.soil_to_fertilizer_map.soil_to_fertilizer(soil))
            .map(|fertilizer| self.fertilizer_to_water_map.fertilizer_to_water(fertilizer))
            .map(|water| self.water_to_light_map.water_to_light(water))
            .map(|light| self.light_to_temperature_map.light_to_temperature(light))
            .map(|temperature| {
                self.temperature_to_humidity_map
                    .temperature_to_humidity(temperature)
            })
            .map(|humidity| self.humidity_to_location_map.humidity_to_location(humidity))
            .min_by(|location1, location2| location1.cmp(location2))
            .unwrap()
    }

    fn extract_seeds_from_almanac(almanac: &str) -> Vec<u64> {
        almanac
            .split('\n')
            .filter(|line| line.to_lowercase().contains("seeds:"))
            .collect::<String>()
            .chars()
            .filter(|character| character.is_whitespace() || character.is_ascii_digit())
            .collect::<String>()
            .split_whitespace()
            .map(|digit| digit.trim().parse::<u64>().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_min_location_value(expected_min_location_value: u64, almanac: &str) {
        assert_eq!(
            expected_min_location_value,
            FarmingAlmanac::parse(almanac).get_min_location()
        );
    }

    #[test]
    fn minimal_location_value_from_almanac() {
        assert_min_location_value(35, "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\n\nwater-to-light map:\n88 18 7\n18 25 70\n\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4\n");
    }
}
