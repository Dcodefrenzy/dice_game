use std::io;
use rand::Rng;
fn get_input( input: &mut String)  {
    io::stdin()
        .read_line(input)
            .expect("Please enter something");
}

fn roll_dice() -> u8 {
    
    let dice = rand::thread_rng().gen_range(0..6);
    dice
}
fn main() {
    println!("Play a Dice game with a computer!");
    let dice1 = roll_dice();
    let dice2 = roll_dice();
    println!("dice1 {}, dice2 {}", dice1, dice2);
    let mut roll = String::new();
}
