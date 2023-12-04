#[derive(Debug)]
pub struct Scratchcards {
    cards: Vec<Scratchcard>,
}

impl Scratchcards {
    pub fn parse(scratch_cards: &str) -> Self {
        let mut cards: Vec<Scratchcard> = Vec::new();

        for scratch_card in scratch_cards.trim().split('\n') {
            cards.push(Scratchcard::parse(scratch_card));
        }

        for i in 1..=cards.len() {
            let guessed_numbers_count = cards[i - 1].get_correct_guess_count() as usize;
            for j in 1..=cards.len() {
                if cards[j - 1].id > cards[i - 1].id
                    && cards[j - 1].id <= cards[i + guessed_numbers_count - 1].id
                {
                    cards[j - 1].number_of_copies += cards[i - 1].number_of_copies;
                }
            }
        }

        Scratchcards { cards }
    }

    pub fn points_sum(&self) -> u64 {
        self.cards
            .iter()
            .fold(0, |acc, card| acc + card.get_points())
    }

    pub fn won_cards_count(&self) -> u64 {
        self.cards
            .iter()
            .fold(0, |acc, card| acc + card.number_of_copies)
    }
}

#[derive(Clone, Debug)]
struct Scratchcard {
    id: u64,
    winning_numbers: Vec<u64>,
    scratched_numbers: Vec<u64>,
    number_of_copies: u64,
}

impl Scratchcard {
    fn parse(scratch_card: &str) -> Self {
        let mut winning_numbers: Vec<u64> = Vec::new();
        let mut scratched_numbers: Vec<u64> = Vec::new();

        let mut card_line = scratch_card.split(':');

        let card_id_string = card_line.next().unwrap();
        let card_id_split_index = card_id_string.chars().fold(0, |mut index, char| {
            if char.is_whitespace() {
                index = card_id_string.rfind(char).unwrap();
            }
            index
        });
        let (_, card_id) = card_id_string.split_at(card_id_split_index);

        let card_numbers = card_line.next().unwrap();
        let mut numbers = card_numbers.split('|');

        let winning_numbers_string = numbers.next().unwrap();
        let scratched_numbers_string = numbers.next().unwrap();

        winning_numbers.append(
            &mut winning_numbers_string
                .split_whitespace()
                .map(|str| str.parse().unwrap())
                .collect::<Vec<u64>>(),
        );

        scratched_numbers.append(
            &mut scratched_numbers_string
                .split_whitespace()
                .map(|str| str.trim().parse().unwrap())
                .collect::<Vec<u64>>(),
        );

        Scratchcard {
            id: card_id.trim().parse::<u64>().unwrap(),
            winning_numbers,
            scratched_numbers,
            number_of_copies: 1,
        }
    }

    fn get_points(&self) -> u64 {
        let number_of_matches = self
            .winning_numbers
            .iter()
            .filter(|winning_number| self.scratched_numbers.contains(winning_number))
            .count();

        if number_of_matches == 0 {
            0
        } else {
            2_u64.pow(number_of_matches as u32 - 1)
        }
    }

    pub fn get_correct_guess_count(&self) -> u64 {
        self.winning_numbers
            .iter()
            .filter(|winning_number| self.scratched_numbers.contains(winning_number))
            .count() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_scratchcard_points_sum(expected_scratchcard_points_sum: u64, scratchcard_game: &str) {
        assert_eq!(
            expected_scratchcard_points_sum,
            Scratchcards::parse(scratchcard_game).points_sum()
        );
    }

    fn assert_scratchcard_card_sum(expected_scratchcard_card_sum: u64, scratchcard_game: &str) {
        assert_eq!(
            expected_scratchcard_card_sum,
            Scratchcards::parse(scratchcard_game).won_cards_count()
        );
    }

    #[test]
    fn one_scratchcard_no_winning_numbers() {
        assert_scratchcard_points_sum(0, "Card 1: 1 2 3 4 5 | 11 12 13 14 15 16 17 18");
    }

    #[test]
    fn one_scratchcard_one_winning_number() {
        assert_scratchcard_points_sum(1, "Card 2: 1 2 3 4 5 | 1 12 13 14 15 16 17 18");
    }

    #[test]
    fn one_scratchcard_two_winning_numbers() {
        assert_scratchcard_points_sum(2, "Card 3: 1 2 3 4 5 | 1 2 13 14 15 16 17 18");
    }

    #[test]
    fn one_scratchcard_three_winning_numbers() {
        assert_scratchcard_points_sum(4, "Card 4: 1 2 3 4 5 | 1 2 3 14 15 16 17 18");
    }

    #[test]
    fn one_scratchcard_four_winning_numbers() {
        assert_scratchcard_points_sum(8, "Card 5: 1 2 3 4 5 | 1 2 3 4 15 16 17 18");
    }

    #[test]
    fn one_scratchcard_five_winning_numbers() {
        assert_scratchcard_points_sum(16, "Card 6: 1 2 3 4 5 | 1 2 3 4 5 16 17 18");
    }

    #[test]
    fn one_scratchcard_five_winning_numbers_multiple_spaces_before_card_id() {
        assert_scratchcard_points_sum(16, "Card   6: 1 2 3 4 5 | 1 2 3 4 5 16 17 18");
    }

    #[test]
    fn one_scratchcard_five_winning_numbers_multiple_spaces_number() {
        assert_scratchcard_points_sum(16, "Card 6: 1 2 3 4  5 | 1 2   3 4 5 16 17 18");
    }

    #[test]
    fn multi_scratchcards_four_winning_numbers() {
        assert_scratchcard_points_sum(
            4,
            "Card 1: 11 12 3 4 15 | 1 2 3 4 5 16 17 18\nCard 1: 1 12 13 14 5 | 1 2 3 4 5 16 17 18",
        );
    }

    #[test]
    fn multi_scratchcards_thirty_won_cards() {
        assert_scratchcard_card_sum(30, "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")
    }
}
