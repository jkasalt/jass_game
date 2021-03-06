use colored::Colorize;
use std::cmp::Ordering;
pub const ALL_CARDS: [Card; 36] = [
    //Clubs
    Card {
        number: Number::Six,
        suit: Suit::Clubs,
    },
    Card {
        number: Number::Seven,
        suit: Suit::Clubs,
    },
    Card {
        number: Number::Eight,
        suit: Suit::Clubs,
    },
    Card {
        number: Number::Nine,
        suit: Suit::Clubs,
    },
    Card {
        number: Number::Ten,
        suit: Suit::Clubs,
    },
    Card {
        number: Number::Jack,
        suit: Suit::Clubs,
    },
    Card {
        number: Number::Queen,
        suit: Suit::Clubs,
    },
    Card {
        number: Number::King,
        suit: Suit::Clubs,
    },
    Card {
        number: Number::Ace,
        suit: Suit::Clubs,
    },
    //Diamonds
    Card {
        number: Number::Six,
        suit: Suit::Diamonds,
    },
    Card {
        number: Number::Seven,
        suit: Suit::Diamonds,
    },
    Card {
        number: Number::Eight,
        suit: Suit::Diamonds,
    },
    Card {
        number: Number::Nine,
        suit: Suit::Diamonds,
    },
    Card {
        number: Number::Ten,
        suit: Suit::Diamonds,
    },
    Card {
        number: Number::Jack,
        suit: Suit::Diamonds,
    },
    Card {
        number: Number::Queen,
        suit: Suit::Diamonds,
    },
    Card {
        number: Number::King,
        suit: Suit::Diamonds,
    },
    Card {
        number: Number::Ace,
        suit: Suit::Diamonds,
    },
    //Hearts
    Card {
        number: Number::Six,
        suit: Suit::Hearts,
    },
    Card {
        number: Number::Seven,
        suit: Suit::Hearts,
    },
    Card {
        number: Number::Eight,
        suit: Suit::Hearts,
    },
    Card {
        number: Number::Nine,
        suit: Suit::Hearts,
    },
    Card {
        number: Number::Ten,
        suit: Suit::Hearts,
    },
    Card {
        number: Number::Jack,
        suit: Suit::Hearts,
    },
    Card {
        number: Number::Queen,
        suit: Suit::Hearts,
    },
    Card {
        number: Number::King,
        suit: Suit::Hearts,
    },
    Card {
        number: Number::Ace,
        suit: Suit::Hearts,
    },
    //Spades
    Card {
        number: Number::Six,
        suit: Suit::Spades,
    },
    Card {
        number: Number::Seven,
        suit: Suit::Spades,
    },
    Card {
        number: Number::Eight,
        suit: Suit::Spades,
    },
    Card {
        number: Number::Nine,
        suit: Suit::Spades,
    },
    Card {
        number: Number::Ten,
        suit: Suit::Spades,
    },
    Card {
        number: Number::Jack,
        suit: Suit::Spades,
    },
    Card {
        number: Number::Queen,
        suit: Suit::Spades,
    },
    Card {
        number: Number::King,
        suit: Suit::Spades,
    },
    Card {
        number: Number::Ace,
        suit: Suit::Spades,
    },
];

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Number {
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

//Order is derived only for DISPLAY has nothing to do with the power in-game
#[derive(PartialOrd, Ord, PartialEq, Hash, Eq, Debug, Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

//Order is derived only for DISPLAY has nothing to do with the power in-game
#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub number: Number,
}

impl Card {
    //pub fn jass_cmp(&self, other: &Self, trump: Suit, bottom: Suit) -> Ordering {
    //    if self.suit == trump {
    //        if other.suit == trump {
    //            self.power(trump, bottom).cmp(&other.power(trump, bottom))
    //        } else {
    //            Ordering::Greater
    //        }
    //    } else if self.suit == bottom {
    //        if other.suit == bottom {
    //            self.power(trump, bottom).cmp(&other.power(trump, bottom))
    //        } else {
    //            Ordering::Greater
    //        }
    //    } else {
    //        self.power(trump, bottom).cmp(&other.power(trump, bottom))
    //    }
    //}

    pub fn value(&self, trump: Suit) -> u8 {
        match self.number {
            Number::Six => 0,
            Number::Seven => 0,
            Number::Eight => 0,
            Number::Nine => {
                if self.suit == trump {
                    14
                } else {
                    0
                }
            }
            Number::Ten => 10,
            Number::Jack => {
                if self.suit == trump {
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

    pub fn power(&self, trump: Suit, bottom: Suit) -> u8 {
        if self.suit != bottom && self.suit != trump {
            return 0;
        }
        let trump_bonus: u8;
        if self.suit == trump {
            trump_bonus = 9;
        } else {
            trump_bonus = 0;
        }
        match self.number {
            Number::Six => 1 + trump_bonus,
            Number::Seven => 2 + trump_bonus,
            Number::Eight => 3 + trump_bonus,
            Number::Nine => {
                if self.suit == trump {
                    19
                } else {
                    4
                }
            }
            Number::Ten => 5 + trump_bonus,
            Number::Jack => {
                if self.suit == trump {
                    20
                } else {
                    6
                }
            }
            Number::Queen => 7 + trump_bonus,
            Number::King => 8 + trump_bonus,
            Number::Ace => 9 + trump_bonus,
        }
    }
    pub fn display(&self) -> String {
        let s: char = match self.suit {
            Suit::Spades => '♠',
            Suit::Hearts => '♥',
            Suit::Diamonds => '♦',
            Suit::Clubs => '♣',
        };
        let n: char = match self.number {
            Number::Ace => 'A',
            Number::King => 'K',
            Number::Queen => 'Q',
            Number::Jack => 'J',
            Number::Ten => 'T',
            Number::Nine => '9',
            Number::Eight => '8',
            Number::Seven => '7',
            Number::Six => '6',
        };
        format!("[ {}{} ]", n, s)
    }
}

#[cfg(test)]
mod card_tests {
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
        let values: Vec<u8> = v.iter().map(|card| card.value(trump)).collect();
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
        let values: Vec<u8> = v.iter().map(|card| card.value(trump)).collect();
        assert_eq!(values, vec![0, 0, 0, 14, 10, 20, 3, 4, 11]);
    }

    #[test]
    fn card_correct_power_nontrump() {
        let trump = Suit::Diamonds;
        let bottom = Suit::Spades;

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
        let values: Vec<u8> = v.iter().map(|card| card.power(trump, bottom)).collect();
        assert_eq!(values, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }
    #[test]
    fn card_correct_power_trump() {
        let trump = Suit::Spades;
        let bottom = trump;

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
        let values: Vec<u8> = v.iter().map(|card| card.power(trump, bottom)).collect();
        assert_eq!(values, vec![0, 1, 2, 10, 4, 11, 6, 7, 8]);
    }

    #[test]
    fn card_order() {
        let big_spade = Card {
            number: Number::Ace,
            suit: Suit::Spades,
        };
        let medium_spade = Card {
            number: Number::Nine,
            suit: Suit::Spades,
        };
        let small_spade = Card {
            number: Number::Six,
            suit: Suit::Spades,
        };
        let other_diamond = Card {
            number: Number::Ten,
            suit: Suit::Diamonds,
        };
        let other_club = Card {
            number: Number::Ten,
            suit: Suit::Clubs,
        };

        assert!(big_spade > medium_spade);
        assert!(!(big_spade < medium_spade));
        assert!(medium_spade > small_spade);
        assert!(small_spade > other_diamond);
        assert!(other_diamond > other_club);
    }
}
