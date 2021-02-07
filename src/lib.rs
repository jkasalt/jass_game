use std::collections::HashSet;
#[derive(Hash)]
enum Number {
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Hash)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Hash)]
struct Card {
    number: Number,
    suit: Suit,
}

struct Hand(HashSet<Card>);

impl Card {
    fn value(&self, trump: &Suit) -> u8 {
        match &self.number {
            Number::Six => 0,
            Number::Seven => 0,
            Number::Eight => 0,
            Number::Nine => {
                if &self.suit == trump {
                    14
                } else {
                    0
                }
            }
            Number::Ten => 10,
            Number::Jack => {
                if &self.suit == trump {
                    20
                } else {
                    2
                }
            }
            Number::Queen => 3,
            Number::King => 4,
            Number::Ace => 11,
        }
    }

    fn power(&self, trump: &Suit) -> u8 {
        match &self.number {
            Number::Six => 0,
            Number::Seven => 1,
            Number::Eight => 2,
            Number::Nine => {
                if &self.suit == trump {
                    10
                } else {
                    3
                }
            }
            Number::Ten => 4,
            Number::Jack => {
                if &self.suit == trump {
                    11
                } else {
                    5
                }
            }
            Number::Queen => 6,
            Number::King => 7,
            Number::Ace => 8,
        }
    }
}

impl Hand {
    fn play(&self, card: &Card) -> Result<Card, &'static str> {}
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    #[test]
    fn card_correct_value_nontrump() {
        let trump = Suit::Diamonds;

        let card6 = Card {
            suit: Suit::Spades,
            number: Number::Six,
        };
        let card7 = Card {
            suit: Suit::Spades,
            number: Number::Seven,
        };
        let card8 = Card {
            suit: Suit::Spades,
            number: Number::Eight,
        };
        let card9 = Card {
            suit: Suit::Spades,
            number: Number::Nine,
        };
        let card_t = Card {
            suit: Suit::Spades,
            number: Number::Ten,
        };
        let card_j = Card {
            suit: Suit::Spades,
            number: Number::Jack,
        };
        let card_q = Card {
            suit: Suit::Spades,
            number: Number::Queen,
        };
        let card_k = Card {
            suit: Suit::Spades,
            number: Number::King,
        };
        let card_a = Card {
            suit: Suit::Spades,
            number: Number::Ace,
        };
        let v = vec![
            card6, card7, card8, card9, card_t, card_j, card_q, card_k, card_a,
        ];
        let values: Vec<u8> = v.iter().map(|card| card.value(&trump)).collect();
        assert_eq!(values, vec![0, 0, 0, 0, 10, 2, 3, 4, 11]);
    }
    #[test]
    fn card_correct_value_trump() {
        let trump = Suit::Diamonds;
        let card6 = Card {
            suit: Suit::Diamonds,
            number: Number::Six,
        };
        let card7 = Card {
            suit: Suit::Diamonds,
            number: Number::Seven,
        };
        let card8 = Card {
            suit: Suit::Diamonds,
            number: Number::Eight,
        };
        let card9 = Card {
            suit: Suit::Diamonds,
            number: Number::Nine,
        };
        let card_t = Card {
            suit: Suit::Diamonds,
            number: Number::Ten,
        };
        let card_j = Card {
            suit: Suit::Diamonds,
            number: Number::Jack,
        };
        let card_q = Card {
            suit: Suit::Diamonds,
            number: Number::Queen,
        };
        let card_k = Card {
            suit: Suit::Diamonds,
            number: Number::King,
        };
        let card_a = Card {
            suit: Suit::Diamonds,
            number: Number::Ace,
        };

        let v = vec![
            card6, card7, card8, card9, card_t, card_j, card_q, card_k, card_a,
        ];
        let values: Vec<u8> = v.iter().map(|card| card.value(&trump)).collect();
        assert_eq!(values, vec![0, 0, 0, 14, 10, 20, 3, 4, 11]);
    }

    #[test]
    fn card_correct_power_nontrump() {
        let trump = Suit::Diamonds;

        let card6 = Card {
            suit: Suit::Spades,
            number: Number::Six,
        };
        let card7 = Card {
            suit: Suit::Spades,
            number: Number::Seven,
        };
        let card8 = Card {
            suit: Suit::Spades,
            number: Number::Eight,
        };
        let card9 = Card {
            suit: Suit::Spades,
            number: Number::Nine,
        };
        let card_t = Card {
            suit: Suit::Spades,
            number: Number::Ten,
        };
        let card_j = Card {
            suit: Suit::Spades,
            number: Number::Jack,
        };
        let card_q = Card {
            suit: Suit::Spades,
            number: Number::Queen,
        };
        let card_k = Card {
            suit: Suit::Spades,
            number: Number::King,
        };
        let card_a = Card {
            suit: Suit::Spades,
            number: Number::Ace,
        };
        let v = vec![
            card6, card7, card8, card9, card_t, card_j, card_q, card_k, card_a,
        ];
        let values: Vec<u8> = v.iter().map(|card| card.power(&trump)).collect();
        assert_eq!(values, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }
    #[test]
    fn card_correct_power_trump() {
        let trump = Suit::Spades;

        let card6 = Card {
            suit: Suit::Spades,
            number: Number::Six,
        };
        let card7 = Card {
            suit: Suit::Spades,
            number: Number::Seven,
        };
        let card8 = Card {
            suit: Suit::Spades,
            number: Number::Eight,
        };
        let card9 = Card {
            suit: Suit::Spades,
            number: Number::Nine,
        };
        let card_t = Card {
            suit: Suit::Spades,
            number: Number::Ten,
        };
        let card_j = Card {
            suit: Suit::Spades,
            number: Number::Jack,
        };
        let card_q = Card {
            suit: Suit::Spades,
            number: Number::Queen,
        };
        let card_k = Card {
            suit: Suit::Spades,
            number: Number::King,
        };
        let card_a = Card {
            suit: Suit::Spades,
            number: Number::Ace,
        };
        let v = vec![
            card6, card7, card8, card9, card_t, card_j, card_q, card_k, card_a,
        ];
        let values: Vec<u8> = v.iter().map(|card| card.power(&trump)).collect();
        assert_eq!(values, vec![0, 1, 2, 10, 4, 11, 6, 7, 8]);
    }
}
