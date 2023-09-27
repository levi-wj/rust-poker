mod cards;
mod game;
mod actions;
mod players;
mod input;

/*fn get_cur_player(game: &Game, players: &Vec<players::Player>) -> &players::Player {
    &players[game.player_turn]
}*/

fn main() {
    const PLAYERCOUNT: usize = 4;
    let mut players: Vec<players::Player> = players::setup_players(PLAYERCOUNT);
    let mut game = game::Game {
        player_turn: 0,
        cur_bet: 0,
        phase: game::GamePhase::Start,
    };

    println!("Welcome to Texas Hold'em poker.");
    println!("You will compete against {} bots to try to win all the chips.", PLAYERCOUNT - 1);

    // Main game loop
    while !game::is_game_over() {
        let mut deck = cards::gen_deck();

        game::start_round(&mut game);
        players::deal(&mut deck, &mut players);

        cards::display_hand(&players[0].hand);
        actions::do_action(&mut game, &mut players);
    }
}
