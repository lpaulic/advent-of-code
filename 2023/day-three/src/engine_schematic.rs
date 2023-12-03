pub struct EngineSchematic {
    part_numbers: Vec<u64>,
    gear_ratios: Vec<u64>,
}

#[derive(Debug)]
struct SchematicSymbol {
    symbol: char,
    row: usize,
    column: usize,
}

#[derive(Debug)]
struct SchematicNumber {
    number: u64,
    row: usize,
    column: usize,
}

impl EngineSchematic {
    pub fn parse(schematic: &str) -> Self {
        let mut part_numbers: Vec<u64> = Vec::new();
        let mut gear_ratios: Vec<u64> = Vec::new();
        let schematic_symbols = EngineSchematic::extract_symbol_locations(schematic);
        let schematic_numbers = EngineSchematic::extract_numbers_with_locations(schematic);

        for schematic_number in &schematic_numbers {
            if schematic_symbols.iter().any(|schematic_symbol| {
                EngineSchematic::is_part_number(schematic_symbol, schematic_number)
            }) {
                part_numbers.push(schematic_number.number);
            }
        }

        for schematic_symbol in schematic_symbols {
            if !schematic_symbol.symbol.eq_ignore_ascii_case(&'*') {
                continue;
            }

            let gear_part_numbers: Vec<u64> = schematic_numbers
                .iter()
                .filter(|schematic_number| {
                    EngineSchematic::is_part_number(&schematic_symbol, schematic_number)
                })
                .map(|schematic_number| schematic_number.number)
                .collect();

            if gear_part_numbers.len() == 2 {
                gear_ratios
                    .push(gear_part_numbers.first().unwrap() * gear_part_numbers.last().unwrap());
            }
        }

        EngineSchematic {
            part_numbers,
            gear_ratios,
        }
    }

    pub fn part_number_sum(&self) -> u64 {
        self.part_numbers.iter().sum()
    }

    pub fn gear_ratio_sum(&self) -> u64 {
        self.gear_ratios.iter().sum()
    }

    fn extract_symbol_locations(schematic: &str) -> Vec<SchematicSymbol> {
        const SYMBOLS: &str = "=$*#+-/@%&";
        let mut symbol_location: Vec<SchematicSymbol> = Vec::new();

        for (line_index, schematic_line) in schematic.split('\n').enumerate() {
            for (character_index, character) in schematic_line.chars().enumerate() {
                if SYMBOLS.contains(character) {
                    symbol_location.push(SchematicSymbol {
                        symbol: character,
                        row: line_index,
                        column: character_index,
                    });
                }
            }
        }

        symbol_location
    }

    fn extract_numbers_with_locations(schematic: &str) -> Vec<SchematicNumber> {
        let mut schematic_numbers: Vec<SchematicNumber> = Vec::new();
        let mut number_digits: Vec<char> = Vec::new();
        let mut number_location: (usize, usize) = (0, 0);

        for (line_index, schematic_line) in schematic.split('\n').enumerate() {
            for (character_index, character) in schematic_line.chars().enumerate() {
                if character.is_ascii_digit() {
                    if number_digits.is_empty() {
                        number_location = (line_index, character_index);
                    }

                    number_digits.push(character);
                } else if !number_digits.is_empty() {
                    let number: u64 = number_digits
                        .iter()
                        .cloned()
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    schematic_numbers.push(SchematicNumber {
                        number,
                        row: number_location.0,
                        column: number_location.1,
                    });
                    number_digits.resize(0, '0');
                }
            }

            if !number_digits.is_empty() {
                let number: u64 = number_digits
                    .iter()
                    .cloned()
                    .collect::<String>()
                    .parse()
                    .unwrap();

                schematic_numbers.push(SchematicNumber {
                    number,
                    row: number_location.0,
                    column: number_location.1,
                });
                number_digits.resize(0, '0');
            }
        }

        schematic_numbers
    }

    fn is_part_number(
        schematic_symbol: &SchematicSymbol,
        schematic_number: &SchematicNumber,
    ) -> bool {
        let number_last_digit_index = schematic_number.number.to_string().len() - 1;

        let is_number_in_adjacent_or_same_row_as_symbol =
            (schematic_symbol.row as isize - schematic_number.row as isize).abs() <= 1;

        let is_number_in_same_column_as_symbol = schematic_symbol.column >= schematic_number.column
            && schematic_symbol.column <= schematic_number.column + number_last_digit_index;

        let is_number_in_adjacent_column_to_symbol = schematic_symbol.column + 1
            == schematic_number.column
            || schematic_symbol.column == schematic_number.column + number_last_digit_index + 1;

        is_number_in_adjacent_or_same_row_as_symbol
            && (is_number_in_same_column_as_symbol || is_number_in_adjacent_column_to_symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_part_number_sum(expected_part_number_sum: u64, engine_schematic: &str) {
        assert_eq!(
            expected_part_number_sum,
            EngineSchematic::parse(engine_schematic).part_number_sum()
        );
    }

    fn assert_gear_ration_sum(expected_gear_ratio_sum: u64, engine_schematic: &str) {
        assert_eq!(
            expected_gear_ratio_sum,
            EngineSchematic::parse(engine_schematic).gear_ratio_sum()
        );
    }

    #[test]
    fn engine_schematic_one_line_contains_only_one_number() {
        assert_part_number_sum(0, "1");
        assert_part_number_sum(2, "=2");
        assert_part_number_sum(3, "3=");
        assert_part_number_sum(4, "$4");
        assert_part_number_sum(5, "5$");
        assert_part_number_sum(6, "*6");
        assert_part_number_sum(7, "7*");
        assert_part_number_sum(8, "#8");
        assert_part_number_sum(9, "9#");
        assert_part_number_sum(10, "+10");
        assert_part_number_sum(11, "11+");
        assert_part_number_sum(12, "-12");
        assert_part_number_sum(13, "13-");
        assert_part_number_sum(14, "/14");
        assert_part_number_sum(15, "15/");
        assert_part_number_sum(16, "@16");
        assert_part_number_sum(17, "17@");
        assert_part_number_sum(18, "%18");
        assert_part_number_sum(19, "19%");
    }

    #[test]
    fn engine_schematic_one_line_contains_two_numbers() {
        assert_part_number_sum(0, "1.1");
        assert_part_number_sum(0, ".1.1.");
        assert_part_number_sum(1, "$1.2");
        assert_part_number_sum(1, "1*.2");
        assert_part_number_sum(2, "1.*2");
        assert_part_number_sum(2, "1.2*");
        assert_part_number_sum(0, "1.2.*");
        assert_part_number_sum(0, "*.1.2");
        assert_part_number_sum(3, "*1.2*");
        assert_part_number_sum(3, "*1*.2*");
        assert_part_number_sum(3, "*1.*2*");
        assert_part_number_sum(3, "*1*2*");
        assert_part_number_sum(3, "1*2");
    }

    #[test]
    fn engine_schematic_one_line_contains_multiple_numbers() {
        assert_part_number_sum(0, "...1....1..12...");
        assert_part_number_sum(13, "*1....1..12*");
        assert_part_number_sum(4, "*1....1..1*2");
    }

    #[test]
    fn engine_schematic_multi_line_only_one_number() {
        assert_part_number_sum(0, "1\n1");
        assert_part_number_sum(1, "$\n1");
        assert_part_number_sum(0, "$...\n...1");
        assert_part_number_sum(1, "$...\n1...");
        assert_part_number_sum(2, "$...\n.2..");
        assert_part_number_sum(5, "2...\n.$..\n..3.");
    }

    #[test]
    fn engine_schematic_multi_line_multiple_numbers() {
        assert_part_number_sum(4361, "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n");
    }

    #[test]
    fn engine_schematic_gear_ration() {
        assert_gear_ration_sum(6, "3*2");
        assert_gear_ration_sum(9, "3*3");
        assert_gear_ration_sum(467835, "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n");
    }
}
