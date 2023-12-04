#[derive(Debug, PartialEq)]
pub struct Calibration {
    values: Vec<u64>,
}

impl Calibration {
    pub fn parse(calibration_values: &str) -> Self {
        let mut calibration = Calibration { values: Vec::new() };

        let calibration_lines = calibration_values.split('\n');
        for line in calibration_lines {
            calibration.values.push(Calibration::parse_line(line));
        }

        calibration
    }

    fn parse_line(calibration_line: &str) -> u64 {
        let modified_calibration_line =
            Calibration::replace_digit_words_with_digits(calibration_line);

        let only_digits: String = modified_calibration_line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect();
        let number_of_digits = only_digits.len();
        let mut first_last_digit: String = only_digits
            .chars()
            .enumerate()
            .filter(|(i, _)| *i == 0 || *i == number_of_digits - 1)
            .map(|(_, c)| c)
            .collect();

        if first_last_digit.is_empty() {
            first_last_digit.push('0');
        } else if first_last_digit.len() == 1 {
            first_last_digit.push_str(&first_last_digit.to_owned());
        }

        first_last_digit
            .parse::<u64>()
            .expect("Only digits should be left in string")
    }

    fn replace_digit_words_with_digits(line: &str) -> String {
        let mut modified_line = line.to_owned();
        // NOTE: reason why the numbers are surrounded by first and last letter is so consecutive number words don't get lost
        let digit_words_to_number_map = [
            ["one", "o1e"],
            ["two", "t2o"],
            ["three", "t3e"],
            ["four", "f4r"],
            ["five", "f5e"],
            ["six", "s6x"],
            ["seven", "s7n"],
            ["eight", "e8t"],
            ["nine", "n9e"],
        ];

        for digit_map_row in digit_words_to_number_map {
            modified_line = modified_line.replace(digit_map_row[0], digit_map_row[1]);
        }

        modified_line
    }

    pub fn sum(&self) -> u64 {
        self.values.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_calibration_sum(expected_sum: u64, calibration_string: &str) {
        assert_eq!(expected_sum, Calibration::parse(calibration_string).sum());
    }

    #[test]
    fn string_empty() {
        assert_calibration_sum(0, "");
    }

    #[test]
    fn string_with_one_line_contains_no_digits() {
        assert_calibration_sum(0, "test");
    }

    #[test]
    fn string_with_one_line_contains_one_digit_string() {
        assert_calibration_sum(44, "4");
        assert_calibration_sum(0, "0");
    }

    #[test]
    fn string_with_one_line_contains_two_digit_string() {
        assert_calibration_sum(23, "t23");
        assert_calibration_sum(23, "2t3");
        assert_calibration_sum(23, "23t");
        assert_calibration_sum(23, "tt23");
        assert_calibration_sum(23, "2tt3");
        assert_calibration_sum(23, "23tt");
        assert_calibration_sum(23, "t2t3");
        assert_calibration_sum(23, "t23t");
        assert_calibration_sum(23, "tt2tt3tt");
    }

    #[test]
    fn string_with_one_line_contains_three_digit_string() {
        assert_calibration_sum(24, "234");
        assert_calibration_sum(24, "t234");
        assert_calibration_sum(24, "2t34");
        assert_calibration_sum(24, "23t4");
        assert_calibration_sum(24, "234t");
        assert_calibration_sum(24, "t2t3t4t");
        assert_calibration_sum(24, "tt2tt3tt4tt");
    }

    #[test]
    fn string_with_one_line_contains_multiple_digit_string() {
        assert_calibration_sum(45, "4685");
        assert_calibration_sum(45, "t4685");
        assert_calibration_sum(45, "4t685");
        assert_calibration_sum(45, "468t5");
        assert_calibration_sum(45, "4685t");
        assert_calibration_sum(45, "tt4tt6tt8tt5tt");
        assert_calibration_sum(10, "1tt4tt6tt8tt5tt7tt8tt9tt0tt2tt0");
    }

    #[test]
    fn string_with_one_line_contains_one_digit_as_word() {
        assert_calibration_sum(11, "one");
        assert_calibration_sum(11, "one1");
        assert_calibration_sum(11, "oneone");
        assert_calibration_sum(12, "onetwo");
        assert_calibration_sum(85, "eightonefive");
        assert_calibration_sum(14, "onetwo4");
        assert_calibration_sum(14, "testonetwotest4");
        assert_calibration_sum(83, "eighthree");
        assert_calibration_sum(79, "sevenine");
    }

    #[test]
    fn string_with_multiple_lines_contains_one_digit_strings() {
        assert_calibration_sum(11, "0\n1");
        assert_calibration_sum(22, "1\n1");
        assert_calibration_sum(22, "1\n1");
        assert_calibration_sum(44, "1\n1\n2");
        assert_calibration_sum(22, "te1st\nte1st");
        assert_calibration_sum(44, "te1st\nte1set\ntest2");
    }

    #[test]
    fn string_with_multiple_lines_contains_no_digits_strings() {
        assert_calibration_sum(0, "test\ntest\ntest");
    }

    #[test]
    fn string_with_multiple_line_contains_two_digit_strings() {
        assert_calibration_sum(21, "10\n1");
        assert_calibration_sum(33, "12\n21");
        assert_calibration_sum(67, "12\n21\n34");
        assert_calibration_sum(21, "t1e0st\ntest1");
        assert_calibration_sum(33, "t1es2t\nte2st1");
        assert_calibration_sum(67, "12test\nts21st\nte3st4");
    }

    #[test]
    fn string_with_multiple_line_contains_no_digit_constraint_strings() {
        assert_calibration_sum(33, "12test\nts21st\ntest");
        assert_calibration_sum(33, "12test\nts21st\ntest");
    }

    #[test]
    fn string_with_multiple_lines_contains_one_digit_as_word() {
        assert_calibration_sum(11, "one");
        assert_calibration_sum(22, "one\n1");
        assert_calibration_sum(33, "one\ntwo");
        assert_calibration_sum(77, "one\ntwo\n4");
        assert_calibration_sum(77, "testone\ntwotest\n4");
    }
}
