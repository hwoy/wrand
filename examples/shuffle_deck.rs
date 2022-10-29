#![allow(dead_code)]

extern crate wrand;
use wrand::{rng, Gen};

use std::collections::LinkedList;
use std::string::String;

#[derive(Clone)]
struct Suit {
    id: u32,
    name: String,
    value: u32,
}

#[derive(Clone)]
struct Rank {
    id: u32,
    name: String,
    value: u32,
}

#[derive(Clone)]
struct Card {
    id: u32,
    suit: Suit,
    rank: Rank,
}

#[derive(Clone)]
struct Deck {
    deck: LinkedList<Card>,
}

impl Deck {
    fn default_suits_ranks() -> ([Suit; 4], [Rank; 13]) {
        let suits: [Suit; 4] = [
            Suit {
                id: 0,
                name: "Clubs".to_string(),
                value: 0,
            },
            Suit {
                id: 1,
                name: "Diamonds".to_string(),
                value: 1,
            },
            Suit {
                id: 2,
                name: "Hearts".to_string(),
                value: 2,
            },
            Suit {
                id: 3,
                name: "Spades".to_string(),
                value: 3,
            },
        ];

        let ranks: [Rank; 13] = [
            Rank {
                id: 0,
                name: "A".to_string(),
                value: 0,
            },
            Rank {
                id: 0,
                name: "2".to_string(),
                value: 0,
            },
            Rank {
                id: 0,
                name: "3".to_string(),
                value: 0,
            },
            Rank {
                id: 0,
                name: "4".to_string(),
                value: 0,
            },
            Rank {
                id: 0,
                name: "5".to_string(),
                value: 0,
            },
            Rank {
                id: 0,
                name: "6".to_string(),
                value: 0,
            },
            Rank {
                id: 0,
                name: "7".to_string(),
                value: 0,
            },
            Rank {
                id: 0,
                name: "8".to_string(),
                value: 0,
            },
            Rank {
                id: 0,
                name: "9".to_string(),
                value: 0,
            },
            Rank {
                id: 0,
                name: "10".to_string(),
                value: 0,
            },
            Rank {
                id: 0,
                name: "J".to_string(),
                value: 0,
            },
            Rank {
                id: 0,
                name: "Q".to_string(),
                value: 0,
            },
            Rank {
                id: 0,
                name: "K".to_string(),
                value: 0,
            },
        ];

        (suits, ranks)
    }

    fn with_suits_ranks(suits: &[Suit], ranks: &[Rank]) -> Self {
        let mut deck: LinkedList<Card> = LinkedList::default();
        let mut i = 0u32;

        for suit in suits.iter() {
            for rank in ranks.iter() {
                deck.push_back(Card {
                    id: i,
                    suit: suit.clone(),
                    rank: rank.clone(),
                });
                i += 1;
            }
        }

        Self { deck: deck }
    }

    fn new() -> Self {
        let (suits, ranks) = Self::default_suits_ranks();
        Self::with_suits_ranks(&suits, &ranks)
    }

    fn shuffle<T: Gen<Output = wrand::RANDTYPE>>(&mut self, gen: &mut T, n: usize) {
        for _ in 0..n {
            let mut tmpdeck = LinkedList::<Card>::default();
            while !self.deck.is_empty() {
                let size = wrand::random(gen.gen(), 0, (self.deck.len() - 1) as wrand::RANDTYPE)
                    .unwrap() as usize;
                let mut splitdeck = self.deck.split_off(size as usize);

                tmpdeck.push_back(splitdeck.pop_front().unwrap());
                self.deck.append(&mut splitdeck);
            }
            self.deck.append(&mut tmpdeck);
        }
    }
}

fn main() {
    let mut rng = rng::Lgcmsvcrt::new();

    let mut maindeck = Deck::new();

    for (i, card) in maindeck.deck.iter().enumerate() {
        println!("{}:{}[{}]", i, card.suit.name, card.rank.name);
    }

    println!("\n================= Shuffle =================\n");
    maindeck.shuffle(&mut rng, 1024);

    for (i, card) in maindeck.deck.iter().enumerate() {
        println!("{}:{}[{}]", i, card.suit.name, card.rank.name);
    }
}
