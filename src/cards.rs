use rand::thread_rng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
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

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

pub fn gen_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    for suit in Suit::iter() {
        for rank in Rank::iter() {
            deck.push(Card { rank: rank, suit: suit });
        }
    }

    deck.shuffle(&mut thread_rng());
    deck
}
