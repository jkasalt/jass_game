use colored::Colorize;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{io, ops::RangeInclusive};
mod card;
use card::*;

#[derive(Clone)]
struct Player {
    hand: Vec<Card>,
    playable_cards: Vec<Card>,
    name: String,
    //TODO implement cacher so that we don't have to recalculate playable_cards() every time
}

#[allow(dead_code)]
impl Player {
    fn new_with_empty_hand(name: String) -> Player {
        Player {
            hand: Vec::new(),
            playable_cards: Vec::new(),
            name,
        }
    }

    fn new(hand: Vec<Card>, name: String) -> Player {
        Player {
            hand,
            name,
            playable_cards: Vec::new(),
        }
    }

    fn discard(&mut self, card: &Card) -> Result<Card, &str> {
        if let Ok(i) = self.hand.binary_search(card) {
            Ok(self.hand.remove(i))
        } else {
            Err("Card not found")
        }
    }

    fn discard_index(&mut self, i: usize) -> Result<Card, &str> {
        if i >= self.hand.len() {
            Err("Error: attempting to discard out of bounds")
        } else {
            Ok(self.hand.remove(i))
        }
    }

    fn display_hand(&self, trump: Suit, bottom: Option<Suit>) {
        println!("{}'s hand:", self.name);
        for card in self.hand.iter() {
            if card.suit == trump && card.number == Number::Jack {
                print!("{}", card.display().white());
            } else if card.suit == trump && self.playable_cards.contains(card) {
                print!("{}", card.display().green());
            } else if self.playable_cards.contains(card) {
                print!("{}", card.display().blue());
            } else {
                print!("{}", card.display().red());
            }
        }
        print!("\n");
    }

    fn update_playable_cards(
        &mut self,
        played_cards: &Vec<Card>,
        trump: Suit,
        bottom: Option<Suit>,
    ) {
        let whole_hand = self.hand.clone();
        let mut trumps_on_table = played_cards
            .clone()
            .into_iter()
            .filter(|x| x.suit == trump)
            .collect::<Vec<Card>>();
        match bottom {
            None => self.playable_cards = whole_hand,
            Some(b) => {
                println!("Asking for suit {:?}", b);
                //If the requested suit is trump and you have one you must play one
                if b == trump {
                    if has_suit(&self.hand, trump) {
                        self.playable_cards =
                            whole_hand.into_iter().filter(|x| x.suit == trump).collect();
                    } else {
                        self.playable_cards = whole_hand;
                    }
                //If the requested suit is not trump...
                } else {
                    trumps_on_table.sort_by_key(|a| a.power(trump, b));
                    let highest_trump_played_power = match trumps_on_table.first() {
                        None => 0,
                        Some(t) => t.power(trump, b),
                    };
                    dbg!(highest_trump_played_power);
                    //If you have the requested suit you may choose
                    //Play a trump higher than the strongest one on the table
                    //Or follow the suit
                    if has_suit(&self.hand, b) {
                        self.playable_cards = whole_hand
                            .into_iter()
                            .filter(|x| {
                                x.suit == b
                                    || (x.suit == trump
                                        && x.power(trump, b) > highest_trump_played_power)
                            })
                            .collect();
                    //If you don't have a suit you may play anything
                    //Except a trump lower that the strongest one on the table
                    } else {
                        self.playable_cards = whole_hand
                            .into_iter()
                            .filter(|x| {
                                x.suit != trump
                                    || (x.suit == trump
                                        && x.power(trump, b) > highest_trump_played_power)
                            })
                            .collect();
                    }
                }
            }
        }
        //In any case you are never forced to play Bour
        let bour_copy = Card {
            suit: trump,
            number: Number::Jack,
        };
        if self.playable_cards.len() == 1 && self.playable_cards.contains(&bour_copy) {
            self.playable_cards = self.hand.clone() //whole_hand
        }
        //print!("{}'s playable cards are:", self.name);
        //display_vec_cards(&self.playable_cards);
    }

    fn play_turn(
        &mut self,
        played_cards: &Vec<Card>,
        trump: Suit,
        bottom: &mut Option<Suit>,
    ) -> Card {
        self.update_playable_cards(played_cards, trump, *bottom);
        loop {
            self.display_hand(trump, *bottom);
            println!("Please select a card (1-{}):", self.hand.len());
            let stdin = io::stdin();
            let mut i = String::new();
            stdin.read_line(&mut i).expect("Failed to read line");
            let i: usize = match i.trim().parse() {
                Ok(0) => continue,
                Ok(n) => n - 1,
                Err(_) => continue,
            };
            let selected_card = match self.hand.get(i) {
                None => {
                    println!("This index, {} is out of bounds", i);
                    continue;
                }
                Some(c) => c,
            };
            if !self.playable_cards.contains(selected_card) {
                println!("this card {} is not playable", selected_card.display());
                continue;
            } else if let None = bottom {
                *bottom = Some(selected_card.suit);
            }
            match self.discard_index(i) {
                Ok(card) => return card,
                Err(s) => {
                    println!("{}", s);
                    continue;
                }
            }
        }
    }
}

fn has_suit(hand: &Vec<Card>, search_suit: Suit) -> bool {
    for card in hand {
        if card.suit == search_suit {
            return true;
        }
    }
    false
}

fn display_vec_cards(cards: &Vec<Card>) {
    for card in cards {
        print!("{}", card.display());
    }
    print!("\n");
}

fn distribute_and_create_players(deck: [Card; 36], names: [String; 4]) -> [Player; 4] {
    let mut hand1 = Vec::new();
    let mut hand2 = Vec::new();
    let mut hand3 = Vec::new();
    let mut hand4 = Vec::new();
    for i in 0..=35 {
        match i / 9 {
            0 => hand1.push(deck[i].clone()),
            1 => hand2.push(deck[i].clone()),
            2 => hand3.push(deck[i].clone()),
            3 => hand4.push(deck[i].clone()),
            _ => unreachable!(),
        };
    }
    hand1.sort_unstable();
    hand2.sort_unstable();
    hand3.sort_unstable();
    hand4.sort_unstable();

    [
        Player::new(hand1, names[0].clone()),
        Player::new(hand2, names[1].clone()),
        Player::new(hand3, names[2].clone()),
        Player::new(hand4, names[3].clone()),
    ]
}

#[derive(Debug, Clone)]
struct TurnInfo {
    card: Card, //in the future we want players to be able to see the last fold played. That's why this is here
    power: u8,
    value: u8,
    index: usize,
}

impl TurnInfo {
    fn new(card: Card, index: usize, trump: Suit, bottom: Suit) -> TurnInfo {
        let power = card.power(trump, bottom);
        let value = card.value(trump);
        TurnInfo {
            card,
            index,
            power,
            value,
        }
    }
}

pub fn play_round() {
    let mut rng = thread_rng();
    let mut deck = ALL_CARDS.clone();
    let mut points_ac: u32 = 0;
    let mut points_bd: u32 = 0;
    deck.shuffle(&mut rng);
    let names = [
        "Alice".to_string(),
        "Bob".to_string(),
        "Charlie".to_string(),
        "Darlene".to_string(),
    ];
    let mut players = distribute_and_create_players(deck, names);
    let mut finished = false;
    let mut idx = 0; //the winner begins the next fold
    let trump_suit = Suit::Spades; //this will need to be selected by the players in game
    while !finished {
        let mut played_cards = Vec::<TurnInfo>::new(); //usize correspond to the index of the player who played this card for example 0 -> player A
        let mut bottom_suit: Option<Suit> = None;
        for i in RangeInclusive::new(0, 3).map(|x| (x + idx) % 4) {
            let tmp_played_cards = played_cards.iter().map(|x| x.card).collect();
            let played_card = players[i].play_turn(&tmp_played_cards, trump_suit, &mut bottom_suit);
            played_cards.push(TurnInfo::new(
                played_card,
                i,
                trump_suit,
                bottom_suit.unwrap(),
            ));
        }
        //find out which played card has the greatest power
        //let mut w: usize = 0;
        //for turn_info in played_cards.iter() {
        //    if turn_info.power > played_cards[w].power {
        //        w = turn_info.index
        //    }
        //}
        //for debug only: show cards played and the one that won
        print!("the played cards were...");
        display_vec_cards(&played_cards.iter().map(|x| x.card).collect());
        //dbg!(&played_cards);

        played_cards.sort_by_key(|x| x.power);
        played_cards.reverse();
        print!("after sorting");
        display_vec_cards(&played_cards.iter().map(|x| x.card).collect());
        //for turn_info in played_cards.iter() {
        //    print!("{}", turn_info.card.display());
        //}
        print!("\n");
        println!(
            "the winner is {}",
            played_cards.first().unwrap().card.display()
        );
        //set the starting index as the winner's
        let w: usize = played_cards.first().unwrap().index;
        idx = w;
        //give the points to the correct team
        //and keep track of MATCH status
        let mut match_ac = true;
        let mut match_bd = true;
        match w {
            0 | 2 => {
                points_ac += played_cards.iter().map(|x| x.value).sum::<u8>() as u32;
                match_bd = false;
            }
            1 | 3 => {
                points_bd += played_cards.iter().map(|x| x.value).sum::<u8>() as u32;
                match_ac = false;
            }
            _ => unreachable!("unreachable statement in play_round() for w"),
        }

        //finally if there are no more cards to play the round is finished
        if players[0].hand.len() <= 0 {
            //cinq de der
            match w {
                0 | 2 => points_ac += 5,
                1 | 3 => points_bd += 5,
                _ => unreachable!("unreachable statement in play_round() for w"),
            };
            //full match bonus
            assert!(!(match_ac && match_bd));
            if match_ac {
                println!("Full match from team AC");
                points_ac += 100;
            }
            if match_bd {
                println!("Full match from team BD");
                points_bd += 100;
            }

            finished = true; //the round is over
        }
    }
    println!(
        "round over -- points_ac: {}  points_bd: {}",
        points_ac, points_bd
    );
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    #[test]
    fn forced_bour() {
        let trump = Suit::Spades;
        let bour = Card {
            suit: trump,
            number: Number::Jack,
        };
        let small = Card {
            suit: Suit::Hearts,
            number: Number::Eight,
        };

        let mut player = Player::new(vec![bour, small], "bob".to_string());
        let bottom = trump;
        player.update_playable_cards(&Vec::<Card>::new(), trump, Some(bottom));

        assert_eq!(player.playable_cards.len(), 2);
    }

    #[test]
    fn shuffled_deck_is_different() {
        let mut deck = Vec::from(ALL_CARDS);
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
        assert_ne!(deck, ALL_CARDS);
    }

    #[test]
    fn hand_play() {
        let a = Card {
            number: Number::Six,
            suit: Suit::Clubs,
        };
        let b = Card {
            number: Number::Six,
            suit: Suit::Spades,
        };
        let hh = vec![a, b];
        let mut player = Player::new(hh, "Jerry".to_string());
        let a_copy = Card {
            number: Number::Six,
            suit: Suit::Clubs,
        };
        let played = player.discard(&a_copy).unwrap();
        assert_eq!(played, a_copy);
        assert_eq!(player.hand.len(), 1);
        let played = player.discard(&a_copy);
        assert_eq!(played, Err("Card not found"));
    }
}
