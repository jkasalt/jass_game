use colored::Colorize;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{io, ops::RangeInclusive};
mod card;
use card::*;

#[derive(Clone)]
struct Player {
    hand: Vec<Card>,
    name: String,
    //TODO implement cacher so that we don't have to recalculate playable_cards() every time
}

#[allow(dead_code)]
impl Player {
    fn new_with_empty_hand(name: String) -> Player {
        Player {
            hand: Vec::new(),
            name,
        }
    }

    fn new(hand: Vec<Card>, name: String) -> Player {
        Player { hand, name }
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
        let playable_cards = self.playable_cards(trump, bottom);
        println!("{}'s hand:", self.name);
        for card in self.hand.iter() {
            if card.suit == trump {
                print!("{}", card.display(trump, bottom).green());
            } else if playable_cards.contains(card) {
                print!("{}", card.display(trump, bottom).blue());
            } else {
                print!("{}", card.display(trump, bottom).red());
            }
        }
        print!("\n");
    }

    fn playable_cards(&self, trump: Suit, bottom: Option<Suit>) -> Vec<Card> {
        //TODO implement sous-coupage rule
        let playable_alone: Vec<Card> = self
            .hand
            .clone()
            .into_iter()
            .filter(|x| x.is_playable_alone(trump, bottom))
            .collect();
        if playable_alone.is_empty() {
            self.hand.clone()
        } else {
            playable_alone
        }
    }

    fn play_turn(&mut self, trump: Suit, bottom: &mut Option<Suit>) -> Card {
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
            if !self.playable_cards(trump, *bottom).contains(&self.hand[i]) {
                println!("this card is not playable");
                continue;
            } else if let None = bottom {
                *bottom = Some(self.hand[i].suit);
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

struct TurnInfo {
    card: Card, //in the future we want players to be able to see the last fold played. That's why this is here
    power: u8,
    value: u8,
    index: usize,
}

impl TurnInfo {
    fn new(card: Card, index: usize, trump: Suit) -> TurnInfo {
        let power = card.power(trump);
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
        dbg!(idx);
        let mut bottom_suit: Option<Suit> = None;
        for i in RangeInclusive::new(0, 3).map(|x| (x + idx) % 4) {
            played_cards.push(TurnInfo::new(
                players[i].play_turn(trump_suit, &mut bottom_suit),
                i,
                trump_suit,
            ));
        }
        //find out which played card has the greatest power
        let mut w: usize = 0;
        for turn_info in played_cards.iter() {
            if turn_info.power > played_cards[w].power {
                w = turn_info.index
            }
        }
        //set the starting index as the winner's
        idx = w;
        //give the points to the correct team
        match w {
            0 | 2 => points_ac += played_cards.iter().map(|x| x.value).sum::<u8>() as u32,
            1 | 3 => points_bd += played_cards.iter().map(|x| x.value).sum::<u8>() as u32,
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
            finished = true;
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
        let mut player = Player {
            hand: hh,
            name: "Jerry".to_string(),
        };
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
