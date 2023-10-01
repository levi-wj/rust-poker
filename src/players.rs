use crate::cards::Card;

const NAMES: [&str; 5] = ["You", "Bot 1", "Bot 2", "Bot 3", "Bot 4"];

#[derive(Debug)]
pub struct Player {
    pub name: &'static str,
    pub chips: i32,
    pub hand: Vec<Card>,
    pub cur_bet: i32,
    pub folded: bool,
}

pub fn setup_players(playercount: usize) -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    for i in 0..playercount {
        players.push(Player {
            name: NAMES[i],
            chips: 250,
            hand: Vec::new(),
            cur_bet: 1,
            folded: false,
        });
    }
    players
}

pub fn deal(deck: &mut Vec<Card>, players: &mut Vec<Player>) {
    for player in players {
        player.hand = vec![deck.pop().unwrap(), deck.pop().unwrap()]
    }
}

pub fn collect_chips(players: &mut Vec<Player>) -> i32 {
    let mut pool: i32 = 0;
    for player in players {
        pool += player.cur_bet;
        player.chips -= player.cur_bet;
    }
    pool
}
