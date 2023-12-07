use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CamelCards {
    hand_bid_list: Vec<HandBidMap>,
}

impl CamelCards {
    pub fn new(list_of_hands: &str) -> Self {
        let mut hand_bid_list: Vec<HandBidMap> = list_of_hands
            .trim()
            .split('\n')
            .map(|line| HandBidMap {
                hand: Hand::from(line.split_whitespace().next().unwrap()),
                bid: line.split_whitespace().nth(1).unwrap().parse().unwrap(),
            })
            .collect();

        hand_bid_list.sort();

        CamelCards { hand_bid_list }
    }

    pub fn get_total_winnings(&self) -> u64 {
        self.hand_bid_list
            .iter()
            .enumerate()
            .map(|(index, hand_bid)| hand_bid.bid * (index as u64 + 1))
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HandBidMap {
    hand: Hand,
    bid: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let cards: Vec<Card> = s.chars().map(Card::from).collect();
        Hand {
            hand_type: Hand::calculate_hand_type(&cards),
            cards,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                // NOTE: assuming they are the same length without check
                for i in 0..self.cards.len() {
                    let order = self.cards[i].cmp(&other.cards[i]);
                    if order.is_lt() || order.is_gt() {
                        return order;
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl Hand {
    fn calculate_hand_type(cards: &[Card]) -> HandType {
        let mut type_count_map = HashMap::from([
            (CardLabel::A(14), 0),
            (CardLabel::K(13), 0),
            (CardLabel::Q(12), 0),
            (CardLabel::J(11), 0),
            (CardLabel::T(10), 0),
            (CardLabel::Nine(9), 0),
            (CardLabel::Eight(8), 0),
            (CardLabel::Seven(7), 0),
            (CardLabel::Six(6), 0),
            (CardLabel::Five(5), 0),
            (CardLabel::Four(4), 0),
            (CardLabel::Three(3), 0),
            (CardLabel::Two(2), 0),
            (CardLabel::One(1), 0),
        ]);

        cards
            .iter()
            .for_each(|card| *type_count_map.get_mut(&card.label).unwrap() += 1);

        let hand_type: HandType;
        if type_count_map.iter().any(|type_count| *type_count.1 == 5) {
            hand_type = HandType::FiveOfAKind(7);
        } else if type_count_map.iter().any(|type_count| *type_count.1 == 4) {
            hand_type = HandType::FourOfAKind(6);
        } else if type_count_map
            .iter()
            .filter(|type_count| *type_count.1 == 2)
            .count()
            == 1
            && type_count_map
                .iter()
                .filter(|type_count| *type_count.1 == 3)
                .count()
                == 1
        {
            hand_type = HandType::FullHouse(5);
        } else if type_count_map.iter().any(|type_count| *type_count.1 == 3) {
            hand_type = HandType::ThreeOfAKind(4);
        } else if type_count_map
            .iter()
            .filter(|type_count| *type_count.1 == 2)
            .count()
            == 2
        {
            hand_type = HandType::TwoPair(3);
        } else if type_count_map.iter().any(|type_count| *type_count.1 == 2) {
            hand_type = HandType::OnePair(2);
        } else {
            hand_type = HandType::HighCard(1);
        }

        hand_type
    }
}

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    FiveOfAKind(u8),
    FourOfAKind(u8),
    FullHouse(u8),
    ThreeOfAKind(u8),
    TwoPair(u8),
    OnePair(u8),
    HighCard(u8),
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_weight = match *self {
            HandType::FiveOfAKind(weight) => weight,
            HandType::FourOfAKind(weight) => weight,
            HandType::FullHouse(weight) => weight,
            HandType::ThreeOfAKind(weight) => weight,
            HandType::TwoPair(weight) => weight,
            HandType::OnePair(weight) => weight,
            HandType::HighCard(weight) => weight,
        };

        let other_weight = match *other {
            HandType::FiveOfAKind(weight) => weight,
            HandType::FourOfAKind(weight) => weight,
            HandType::FullHouse(weight) => weight,
            HandType::ThreeOfAKind(weight) => weight,
            HandType::TwoPair(weight) => weight,
            HandType::OnePair(weight) => weight,
            HandType::HighCard(weight) => weight,
        };

        self_weight.cmp(&other_weight)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    label: CardLabel,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.label.cmp(&other.label)
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        Card {
            label: if c == 'A' || c == 'a' {
                CardLabel::A(14)
            } else if c == 'K' || c == 'k' {
                CardLabel::K(13)
            } else if c == 'Q' || c == 'q' {
                CardLabel::Q(12)
            } else if c == 'J' || c == 'j' {
                CardLabel::J(11)
            } else if c == 'T' || c == 't' {
                CardLabel::T(10)
            } else if c == '9' {
                CardLabel::Nine(9)
            } else if c == '8' {
                CardLabel::Eight(8)
            } else if c == '7' {
                CardLabel::Seven(7)
            } else if c == '6' {
                CardLabel::Six(6)
            } else if c == '5' {
                CardLabel::Five(5)
            } else if c == '4' {
                CardLabel::Four(4)
            } else if c == '3' {
                CardLabel::Three(3)
            } else if c == '2' {
                CardLabel::Two(2)
            } else {
                CardLabel::One(1)
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum CardLabel {
    A(u8),
    K(u8),
    Q(u8),
    J(u8),
    T(u8),
    Nine(u8),
    Eight(u8),
    Seven(u8),
    Six(u8),
    Five(u8),
    Four(u8),
    Three(u8),
    Two(u8),
    One(u8),
}

impl PartialOrd for CardLabel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardLabel {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_weight = match *self {
            CardLabel::A(weight) => weight,
            CardLabel::K(weight) => weight,
            CardLabel::Q(weight) => weight,
            CardLabel::J(weight) => weight,
            CardLabel::T(weight) => weight,
            CardLabel::Nine(weight) => weight,
            CardLabel::Eight(weight) => weight,
            CardLabel::Seven(weight) => weight,
            CardLabel::Six(weight) => weight,
            CardLabel::Five(weight) => weight,
            CardLabel::Four(weight) => weight,
            CardLabel::Three(weight) => weight,
            CardLabel::Two(weight) => weight,
            CardLabel::One(weight) => weight,
        };

        let other_weight = match *other {
            CardLabel::A(weight) => weight,
            CardLabel::K(weight) => weight,
            CardLabel::Q(weight) => weight,
            CardLabel::J(weight) => weight,
            CardLabel::T(weight) => weight,
            CardLabel::Nine(weight) => weight,
            CardLabel::Eight(weight) => weight,
            CardLabel::Seven(weight) => weight,
            CardLabel::Six(weight) => weight,
            CardLabel::Five(weight) => weight,
            CardLabel::Four(weight) => weight,
            CardLabel::Three(weight) => weight,
            CardLabel::Two(weight) => weight,
            CardLabel::One(weight) => weight,
        };

        self_weight.cmp(&other_weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_camel_card_total_winnings(expected_total_winnings: u64, list_of_hands: &str) {
        assert_eq!(
            expected_total_winnings,
            CamelCards::new(list_of_hands).get_total_winnings()
        );
    }

    #[test]
    fn camel_cards_one_hand() {
        assert_camel_card_total_winnings(1, "3456 1");
        assert_camel_card_total_winnings(2, "3456 2");
    }

    #[test]
    fn camel_cards_two_hands_same_type() {
        assert_camel_card_total_winnings(5, "3456 1\n3456 2");
    }

    #[test]
    fn camel_cards_two_hands_different_type() {
        assert_camel_card_total_winnings(4, "AAAAA 1\n3456 2");
        assert_camel_card_total_winnings(4, "AA776 1\nAA775 2");
    }

    #[test]
    fn camel_cards_five_hands_different_types() {
        assert_camel_card_total_winnings(
            6440,
            "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483\n",
        );
    }
}
