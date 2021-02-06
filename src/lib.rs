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

#[derive(PartialEq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

struct Card {
    number: Number,
    suit: Suit,
}

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
            },
            Number::Ten => 10,
            Number::Jack => {
                if &self.suit == trump {
                    20
                } else {
                    2
                }
            },
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
            },
            Number::Ten => 4,
            Number::Jack => {
                if &self.suit == trump {
                    11
                } else {
                    5
                }
            },
            Number::Queen => 6,
            Number::King => 7,
            Number::Ace => 8,
        }
    }
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
        let cardT = Card {
            suit: Suit::Spades,
            number: Number::Ten,
        };
        let cardJ = Card {
            suit: Suit::Spades,
            number: Number::Jack,
        };
        let cardQ = Card {
            suit: Suit::Spades,
            number: Number::Queen,
        };
        let cardK = Card {
            suit: Suit::Spades,
            number: Number::King,
        };
        let cardA = Card {
            suit: Suit::Spades,
            number: Number::Ace,
        };
        let v = vec![card6, card7, card8, card9, cardT, cardJ, cardQ, cardK, cardA];
        let values: Vec<u8> = v
            .iter()
            .map(|card| card.value(&trump))
            .collect();
        assert_eq!(values, vec![0,0,0,0,10,2,3,4,11]);
    }
    #[test]
     fn card_correct_value_trump() {
         let trump = Suit::Diamonds;
         let card_9 = Card {
             suit: Suit::Diamonds,
             number: Number::Nine,
         };
         let card_j = Card {
             suit: Suit::Diamonds,
             number: Number::Jack,
         };

         let v= vec![card_9, card_j];
         let values: Vec<u8> = v.iter().map(|card| card.value(&trump)).collect();
         assert_eq!(values, vec![14, 20]);
     }
}