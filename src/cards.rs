use rand::thread_rng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::collections::HashMap;

#[derive(PartialOrd, PartialEq, Eq, Hash, Debug)]
pub enum HandEval {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Ord, PartialOrd, Hash, Eq)]
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
impl Rank {
    fn next(&self) -> Self {
        match self {
            Rank::Two => Rank::Three,   
            Rank::Three => Rank::Four,
            Rank::Four => Rank::Five,
            Rank::Five => Rank::Six,
            Rank::Six => Rank::Seven,
            Rank::Seven => Rank::Eight,
            Rank::Eight => Rank::Nine,
            Rank::Nine => Rank::Ten,
            Rank::Ten => Rank::Jack,
            Rank::Jack => Rank::Queen,
            Rank::Queen => Rank::King,
            Rank::King => Rank::Ace,
            Rank::Ace => Rank::Two,
        }
    }
    fn get_num(&self) -> i32 {
        match self {
            Rank::Two => 2,   
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone)]
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

fn get_card_display(card: &Card) -> Vec<String> {
    let mut display: Vec<String> = Vec::new();
    let symbol = match card.suit {
        Suit::Hearts => "♥",
        Suit::Diamonds => "♦",
        Suit::Clubs => "♣",
        Suit::Spades => "♠",
    };
    let value = match card.rank {
        Rank::Two => "2",
        Rank::Three => "3",
        Rank::Four => "4",
        Rank::Five => "5",
        Rank::Six => "6",
        Rank::Seven => "7",
        Rank::Eight => "8",
        Rank::Nine => "9",
        Rank::Ten => "10",
        Rank::Jack => "J",
        Rank::Queen => "Q",
        Rank::King => "K",
        Rank::Ace => "A",
    };
    display.push("┌─────────┐".to_string());
    display.push(format!("│{:2}       │", value));
    display.push("│         │".to_string());
    display.push("│         │".to_string());
    display.push(format!("│    {}    │", symbol));
    display.push("│         │".to_string());
    display.push("│         │".to_string());
    display.push(format!("│       {:2}│", value));
    display.push("└─────────┘".to_string());
    display
}

fn display_cards(cards: &Vec<Card>) {
    let mut display: Vec<String> = vec!["".to_string(); 9];

    for card in cards {
        let mut i = 0;
        let cur_card_display: Vec<String> = get_card_display(card);
        for line in &mut display {
            line.push_str(&cur_card_display[i]);
            i += 1;
        }
    }
    for line in display {
        println!("{}", line);
    }
}

pub fn flip_cards(card_count: usize, deck: &mut Vec<Card>) -> Vec<Card> {
    let mut cards = Vec::new();
    for _ in 0..card_count {
        cards.push(deck.pop().unwrap());
    }
    cards
}

pub fn display_hand(hand: &Vec<Card>) {
    println!("\nYour hand:");
    display_cards(hand);
    println!("");
}

pub fn display_table(table: &Vec<Card>) {
    if table.len() > 0 {
        println!("\nCards on the table:");
        display_cards(table);
        println!("");
    }
}

fn get_card_counts(cards: &Vec<&Card>) -> HashMap<Rank, usize> {
    let mut map: HashMap<Rank, usize> = HashMap::new();
    for card in cards {
        *map.entry(card.rank).or_default() += 1;
    }
    map 
}

fn get_pairs(map: &HashMap<Rank, usize>, how_many: usize) -> usize {
    map.into_iter().filter(|(_, x)| **x > (how_many - 1)).count()
}


fn is_straight(cards: &Vec<&Card>) -> bool {
    let mut straightcount: i32 = 0;
    let mut lastcard: Rank = cards[0].rank;

    for card in cards {
        if card.rank == lastcard.next() {
            straightcount += 1;
            lastcard = card.rank;
        } else {
            straightcount = 0;
        }
    }

    straightcount > 4
}

fn is_flush(cards: &Vec<&Card>) -> bool {
    for suit in Suit::iter() {
        let count = cards.iter().filter(|card| card.suit == suit).count();
        if count >= 5 {
            return true;
        }
    }

    false
}

pub fn get_high_card(cards: &Vec<Card>) -> i32 {
    let mut high_card: i32 = 0;
    for card in cards {
        if card.rank.get_num() > high_card {
            high_card = card.rank.get_num();
        }
    }
    high_card
}

pub fn eval_hand(hand: &Vec<Card>, table: &Vec<Card>) -> HandEval {
    let mut eval: HandEval = HandEval::HighCard;
    let mut cards = hand.iter().chain(table.iter()).collect::<Vec<_>>();
    cards.sort_by(|a, b| a.rank.cmp(&b.rank));
    let map: HashMap<Rank, usize> = get_card_counts(&cards);
    let pairs: usize = get_pairs(&map, 2);
    let three_of_kind: usize = get_pairs(&map, 3);
    let four_of_kind: usize = get_pairs(&map, 4);
    let flush = is_flush(&cards);
    let straight = is_straight(&cards);

    if straight && flush {
        eval = HandEval::StraightFlush;
    } else if four_of_kind > 0 {
        eval = HandEval::FourOfAKind;
    } else if three_of_kind > 0 && pairs > 0 {
        eval = HandEval::FullHouse;
    } else if flush {
        eval = HandEval::Flush;
    } else if straight {
        eval = HandEval::Straight;
    } else if three_of_kind > 0 {
        eval = HandEval::ThreeOfAKind;
    } else if pairs > 1 {
        eval = HandEval::TwoPair;
    } else if pairs > 0 {
        eval = HandEval::Pair;
    }

    eval
}
