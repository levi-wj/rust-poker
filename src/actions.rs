use crate::players::Player;
use crate::game::Game;
use crate::cards::Card;
use crate::input::get_user_str;
use crate::input::get_user_int;

pub fn do_action(game: &mut Game, players: &mut Vec<Player>) {
    loop {
        let mut action = get_user_str("\nYour move! You can (r)aise, (c)all, or (f)old.");
        action.truncate(1);

        match action.as_str() {
            "r" => {
                println!("\nYour current bet is {}", players[0].cur_bet);
                let raise: u32 = loop {
                    let input: u32 = get_user_int("How much would you like to raise?");
                    if input > 0 { break input; }
                };
                players[0].cur_bet += raise;
                game.cur_bet = players[0].cur_bet;
                println!("Your bet is now {}", players[0].cur_bet);
                break;
            },
            "c" => {
                println!("Raising your bet to match {}", game.cur_bet);
                players[0].cur_bet = game.cur_bet;
                break;
            },
            "f" => {
                println!("You folded.");
                players[0].folded = true;
                break;
            },
            _ => continue,
        }
    }
}
