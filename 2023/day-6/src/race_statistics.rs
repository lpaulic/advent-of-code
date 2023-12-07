#[derive(Debug)]
pub struct RaceStatistics {
    record_table: Vec<RecordTable>,
}

#[derive(Debug)]
struct RecordTable {
    record_time: u64,
    record_distance: u64,
}

impl RaceStatistics {
    pub fn parse(record_table: &str, ignore_white_space: bool) -> Self {
        let race_time_data = record_table.split('\n').nth(0).unwrap();
        let race_distance_data = record_table.split('\n').nth(1).unwrap();

        let race_times: Vec<u64>;
        let race_distances: Vec<u64>;
        if ignore_white_space {
            race_times = vec![race_time_data
                .split_whitespace()
                .filter(|string| string.parse::<u64>().is_ok())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()];

            race_distances = vec![race_distance_data
                .split_whitespace()
                .filter(|string| string.parse::<u64>().is_ok())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()];
        } else {
            race_times = race_time_data
                .split_whitespace()
                .filter_map(|string| string.parse::<u64>().ok())
                .collect();

            race_distances = race_distance_data
                .split_whitespace()
                .filter_map(|string| string.parse::<u64>().ok())
                .collect();
        }

        // NOTE: assuming race_times.len() == race_distances.len()
        let mut record_table: Vec<RecordTable> = Vec::new();
        for i in 0..race_times.len() {
            record_table.push(RecordTable {
                record_time: race_times[i],
                record_distance: race_distances[i],
            })
        }

        RaceStatistics { record_table }
    }

    pub fn get_race_record_break_product(&self) -> u64 {
        self.record_table
            .iter()
            .map(|entry| {
                RaceStatistics::get_record_break_count(entry.record_time, entry.record_distance)
            })
            .product()
    }

    fn get_record_break_count(time: u64, distance: u64) -> u64 {
        let distance_1 =
            ((time as f64 + ((time.pow(2) - 4 * distance) as f64).sqrt()) / 2_f64).ceil() as u64;
        let distance_2 =
            ((time as f64 - ((time.pow(2) - 4 * distance) as f64).sqrt()) / 2_f64).ceil() as u64;

        (distance_2..distance_1).count() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_race_record_break_product(
        expected_race_record_brake_product: u64,
        record_table: &str,
        ignore_white_space: bool,
    ) {
        assert_eq!(
            expected_race_record_brake_product,
            RaceStatistics::parse(record_table, ignore_white_space).get_race_record_break_product()
        );
    }

    #[test]
    fn one_race_possible_record_breaks() {
        assert_race_record_break_product(4, "Time: 7\nDistance: 9\n", false);
        assert_race_record_break_product(8, "Time: 15\nDistance: 40\n", false);
        assert_race_record_break_product(320, "Time: 7  15   30\nDistance: 9  40  200\n", false);
        assert_race_record_break_product(71503, "Time: 7  15   30\nDistance: 9  40  200\n", true);
    }
}
