#[derive(Debug)]
pub struct SeedMap {
    map: Vec<Seed>,
}

#[derive(Debug)]
pub struct Seed {
    seed_range_start: u64,
    range_size: u64,
}

impl SeedMap {
    pub fn extract_seeds_from_almanac(almanac: &str) -> SeedMap {
        let mut map: Vec<Seed> = Vec::new();

        let seeds: Vec<u64> = almanac
            .split('\n')
            .filter(|line| line.to_lowercase().contains("seeds:"))
            .collect::<String>()
            .chars()
            .filter(|character| character.is_whitespace() || character.is_ascii_digit())
            .collect::<String>()
            .split_whitespace()
            .map(|digit| digit.trim().parse::<u64>().unwrap())
            .collect();

        let mut i = 0;
        while i < seeds.len() {
            map.push(Seed {
                seed_range_start: seeds[i],
                range_size: seeds[i + 1],
            });
            i += 2;
        }

        SeedMap { map }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Seed> {
        self.map.iter()
    }
}

impl Seed {
    pub fn get_range_start(&self) -> u64 {
        self.seed_range_start
    }

    pub fn get_range_end(&self) -> u64 {
        self.seed_range_start + self.range_size
    }
}
