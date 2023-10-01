use crate::cards::Card;

pub struct Game {
    pub player_turn: usize,
    pub cur_bet: i32,
    pub table: Vec<Card>,
}

pub fn start_round(game: &mut Game) {
    game.player_turn = 0;
    game.cur_bet = 0;
    game.table = Vec::new();
}
