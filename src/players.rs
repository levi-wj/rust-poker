use crate::cards::Card;

const NAMES: [&str; 4] = ["You", "Bot 1", "Bot 2", "Bot 3"];

#[derive(Debug)]
pub struct Player {
    pub name: &'static str,
    pub chips: u32,
    pub hand: Vec<Card>,
    pub cur_bet: u32,
}

pub fn setup_players(playercount: usize) -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    for i in 0..playercount {
        players.push(Player {
            name: NAMES[i],
            chips: 100,
            hand: Vec::new(),
            cur_bet: 0,
        });
    }
    players
}
