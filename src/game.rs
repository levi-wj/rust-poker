pub enum GamePhase {
    Start,
    Flop,
    Turn,
    River
}

pub struct Game {
    pub player_turn: usize,
    pub cur_bet: u32,
    pub phase: GamePhase,
}

pub fn start_round(game: &mut Game) {
    game.player_turn = 0;
    game.cur_bet = 0;
    game.phase = GamePhase::Start;
}

pub fn is_game_over() -> bool {
    false
}
