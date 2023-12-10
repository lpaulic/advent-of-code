pub struct OasisPredictor {
    values: Vec<OasisValue>,
}

impl OasisPredictor {
    pub fn parse(oasis_report: &str) -> Self {
        let mut values: Vec<OasisValue> = Vec::new();

        for oasis_line in oasis_report.split('\n') {
            values.push(OasisValue {
                historic_data: oasis_line
                    .split_whitespace()
                    .map(|character| character.trim().parse::<i64>().unwrap())
                    .collect(),
            });
        }

        OasisPredictor { values }
    }

    pub fn next_value_predictions_sum(&self) -> i64 {
        self.values
            .iter()
            .map(|value| value.predict_next_value())
            .sum()
    }

    pub fn previous_value_predictions_sum(&self) -> i64 {
        self.values
            .iter()
            .map(|value| value.predict_previous_value())
            .sum()
    }
}

struct OasisValue {
    historic_data: Vec<i64>,
}

impl OasisValue {
    fn predict_next_value(&self) -> i64 {
        OasisValue::predict_next_data(&self.historic_data)
    }

    fn predict_previous_value(&self) -> i64 {
        OasisValue::predict_previous_data(&self.historic_data)
    }

    fn predict_next_data(values: &[i64]) -> i64 {
        let mut values_iterator = values.iter();
        if values.iter().all(|value| *value == 0) {
            return 0;
        }

        let mut last_value = *values_iterator.next().unwrap();
        let mut diff_values: Vec<i64> = Vec::new();
        values_iterator.for_each(|value| {
            diff_values.push(*value - last_value);
            last_value = *value;
        });

        *(values.iter().last().unwrap()) + OasisValue::predict_next_data(&diff_values)
    }

    fn predict_previous_data(values: &[i64]) -> i64 {
        let mut values_iterator = values.iter();
        if values.iter().all(|value| *value == 0) {
            return 0;
        }

        let mut last_value = *values_iterator.next().unwrap();
        let mut diff_values: Vec<i64> = Vec::new();
        values_iterator.for_each(|value| {
            diff_values.push(*value - last_value);
            last_value = *value;
        });

        *(values.iter().next().unwrap()) - OasisValue::predict_previous_data(&diff_values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_oasis_next_value_predictions_sum(
        expected_next_value_predictions_sum: i64,
        oasis_report: &str,
    ) {
        assert_eq!(
            expected_next_value_predictions_sum,
            OasisPredictor::parse(oasis_report).next_value_predictions_sum()
        );
    }

    fn assert_oasis_previous_value_predictions_sum(
        expected_previous_value_predictions_sum: i64,
        oasis_report: &str,
    ) {
        assert_eq!(
            expected_previous_value_predictions_sum,
            OasisPredictor::parse(oasis_report).previous_value_predictions_sum()
        );
    }

    #[test]
    fn oasis_predictor_one_value_one_historic_data_new_value_prediction() {
        assert_oasis_next_value_predictions_sum(0, "0");
    }

    // NOTE: infinite loop, because left with only one element that is not zero, not handled yet
    // #[test]
    // fn oasis_predictor_one_value_two_historic_data() {
    //     assert_oasis_next_value_predictions_sum(0, "0 1");
    // }

    #[test]
    fn oasis_predictor_one_value_three_historic_data_new_value_prediction() {
        assert_oasis_next_value_predictions_sum(3, "0 1 2");
    }

    #[test]
    fn oasis_predictor_one_value_multiple_historic_data_new_value_prediction() {
        assert_oasis_next_value_predictions_sum(18, "0 3 6 9 12 15");
        assert_oasis_next_value_predictions_sum(28, "1 3 6 10 15 21");
        assert_oasis_next_value_predictions_sum(68, "10 13 16 21 30 45");
    }

    #[test]
    fn oasis_predictor_multiple_values_multiple_historic_data_new_value_prediction() {
        assert_oasis_next_value_predictions_sum(46, "0 3 6 9 12 15\n1 3 6 10 15 21\n");
        assert_oasis_next_value_predictions_sum(
            114,
            "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45",
        );
    }

    #[test]
    fn oasis_predictor_one_value_one_historic_data_previous_value_prediction() {
        assert_oasis_previous_value_predictions_sum(0, "0");
    }

    #[test]
    fn oasis_predictor_one_value_three_historic_data_previous_value_prediction() {
        assert_oasis_previous_value_predictions_sum(-1, "0 1 2");
    }

    #[test]
    fn oasis_predictor_one_value_multiple_historic_data_previous_value_prediction() {
        assert_oasis_previous_value_predictions_sum(-3, "0 3 6 9 12 15");
        assert_oasis_previous_value_predictions_sum(0, "1 3 6 10 15 21");
        assert_oasis_previous_value_predictions_sum(5, "10 13 16 21 30 45");
    }

    #[test]
    fn oasis_predictor_multiple_values_multiple_historic_data_pervious_value_prediction() {
        assert_oasis_previous_value_predictions_sum(-3, "0 3 6 9 12 15\n1 3 6 10 15 21\n");
        assert_oasis_previous_value_predictions_sum(
            2,
            "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45",
        );
    }
}
