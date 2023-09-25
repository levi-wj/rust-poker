pub mod cards;
pub mod players;
pub mod input;

enum GamePhase {
    Start,
    Flop,
    Turn,
    River
}

struct Game {
    player_turn: usize,
    cur_bet: i32,
    pool: i32,
    phase: GamePhase,
}

fn print_intro(player_count: usize) {
    println!("Welcome to Texas Hold'em poker.");
    println!("You will compete against {} bots to try to win all the chips.", player_count - 1);
}

fn deal(deck: &mut Vec<cards::Card>, players: &mut Vec<players::Player>) {
    for player in players {
        player.hand = vec![deck.pop().unwrap(), deck.pop().unwrap()]
    }
}

fn start_round(game: &mut Game) {
    game.player_turn = 0;
    game.cur_bet = 0;
    game.pool = 0;
    game.phase = GamePhase::Start;
}

/*fn get_cur_player(game: &Game, players: &Vec<players::Player>) -> &players::Player {
    &players[game.player_turn]
}*/

fn is_game_over() -> bool {
    false
}

fn main() {
    const PLAYERCOUNT: usize = 4;
    let mut players: Vec<players::Player> = players::setup_players(PLAYERCOUNT);
    let mut game = Game {
        player_turn: 0,
        cur_bet: 0,
        pool: 0,
        phase: GamePhase::Start,
    };

    print_intro(PLAYERCOUNT);

    // Main game loop
    while !is_game_over() {
        let mut deck = cards::gen_deck();

        start_round(&mut game);
        deal(&mut deck, &mut players);

        cards::display_hand(&players[0].hand);

        let mut action = input::get_user_str("\nYour move! You can (r)aise, (c)all, or (f)old.");
        action.truncate(1);

        match action.as_str() {
            "r" => {
                println!("raising");
            },
            "c" => println!("calling"),
            _ => println!("Invalid input."),
        }
    }
}
