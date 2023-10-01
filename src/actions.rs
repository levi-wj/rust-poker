use crate::players::Player;
use crate::game::Game;
use crate::input::get_user_str;
use crate::input::get_user_int;
use rand::Rng;
use crate::cards::get_high_card;

enum Action {
    Raise,
    Call,
    Fold,
}

pub fn do_player_action(game: &mut Game, player: &mut Player)  {
    loop {
        let mut action = get_user_str("\nYour move! You can (r)aise, (c)all/check, or (f)old.");
        action.truncate(1);

        return match action.as_str() {
            "r" => {
                println!("\nYour current bet is {}", player.cur_bet);
                let raise: i32 = loop {
                    let input: i32 = get_user_int("How much would you like to raise?");
                    if input > 0 && input < player.chips { break input; }
                };
                do_action(game, player, Action::Raise, raise);
            },
            "c" => {
                do_action(game, player, Action::Call, 0);
            },
            "f" => {
                do_action(game, player, Action::Fold, 0);
            },
            _ => continue,
        }
    }
}

pub fn do_bot_action(game: &mut Game, player: &mut Player){
    let mut rng = rand::thread_rng();
    let mut confidence: i32 = rng.gen_range(0..200);

    confidence -= (game.cur_bet - player.cur_bet);
    confidence += (get_high_card(&player.hand) * 4);

    if confidence > 120 {
        do_action(game, player, Action::Raise, (rng.gen_range(1..confidence) / 4));
    } else if confidence > 50 {
        do_action(game, player, Action::Call, 0);
    } else {
        do_action(game, player, Action::Fold, 0);
    }
}

fn do_action(game: &mut Game, player: &mut Player, action: Action, amount: i32) {
    match action {
        Action::Raise => {
            game.cur_bet += amount;
            player.cur_bet = game.cur_bet;
            println!("{} raised the bet to {}", player.name, game.cur_bet);
        },
        Action::Call => {
            player.cur_bet = game.cur_bet;
            println!("{} called to match {}", player.name, game.cur_bet);
        },
        Action::Fold => {
            player.folded = true;
            println!("{} folded.", player.name);
        },
    }
}

pub fn play_again() -> bool {
    loop {
        let mut choice = get_user_str("Round over. Would you like to play again (y/n)?");
        choice.truncate(1);
        return match choice.as_str() {
            "y" => true,
            "n" => false,
            _ => continue,
        }
    }
}
