use std::env;

mod cards;
mod game;
mod actions;
mod players;
mod input;

fn do_round(cards_to_flip: usize, game: &mut game::Game, deck: &mut Vec<cards::Card>, players: &mut Vec<players::Player>) {
    cards::display_table(&game.table);
    cards::display_hand(&players[0].hand);
    for player in players {
        if !player.folded {
            if player.name == "You" {
                actions::do_player_action(game, player);
            } else {
                actions::do_bot_action(game, player);
            }
        }
    }
}

fn main() {
    const PLAYERCOUNT: usize = 5;
    let mut players: Vec<players::Player> = players::setup_players(PLAYERCOUNT);
    let mut game = game::Game {
        player_turn: 0,
        cur_bet: 0,
        table: Vec::new(),
    };

    println!("\n\n\nWelcome to Texas Hold'em poker.");
    println!("You will compete against {} bots to try to win all the chips.\n", PLAYERCOUNT - 1);

    // Main game loop
    loop {
        let mut deck = cards::gen_deck();
        let mut best_hand: cards::HandEval = cards::HandEval::HighCard;
        let mut winning_player: usize = 0;
        let mut pool: i32 = 0;

        game::start_round(&mut game);
        players::deal(&mut deck, &mut players);

        println!("You currently have {} chips", players[0].chips);

        // First round
        do_round(0, &mut game, &mut deck, &mut players);

        // Flop
        do_round(3, &mut game, &mut deck, &mut players);

        // Turn
        do_round(1, &mut game, &mut deck, &mut players);

        // River
        do_round(1, &mut game, &mut deck, &mut players);

        let mut player_index: usize = 0;
        for player in &players {
            let hand_eval: cards::HandEval = cards::eval_hand(&player.hand, &game.table);
            if hand_eval > best_hand {
                winning_player = player_index;
                best_hand = hand_eval;
            }
            player_index += 1;
        }

        pool = players::collect_chips(&mut players);
        players[winning_player].chips += pool;
        println!("{} won {} chips with a {:?}.", players[winning_player].name, pool, best_hand);

        if players[0].chips <= 0 {
            println!("\nLooks like you ran out of money! Better luck next time.");
            break;
        }
        if players[0].chips == 1250 {
            println!("\nYou got all the chips! You're the poker champion!");
            break;
        }
        if !actions::play_again() {
            println!("\nThanks for playing!\n");
            break;
        }
    }
}
