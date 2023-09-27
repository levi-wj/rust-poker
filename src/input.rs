use std::io;

pub struct Test {
    pub num: u32
}

pub fn get_user_str(prompt: &str) -> String {
    let mut player_move = String::new();

    println!("{}", prompt);

    io::stdin()
        .read_line(&mut player_move)
        .expect("Failed to read line");

    player_move
}

pub fn get_user_int(prompt: &str) -> u32 {
    let mut guess = String::new();

    println!("{}", prompt);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    guess
}
