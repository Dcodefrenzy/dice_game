use std::io;
use rand::Rng;




fn get_input( input: &mut String)  {
    io::stdin()
        .read_line(input)
            .expect("Please enter something");
}

fn roll_dice() -> u8 {
    
    let dice = rand::thread_rng().gen_range(1..6);
    dice
}

fn start_game() {
        
    let user_dice = roll_dice();
    let computer_dice = roll_dice();
    println!("You rolled {}, Computer rolled {}", user_dice, computer_dice);
    if user_dice > computer_dice {
        println!("You won!");
    } if user_dice == computer_dice {
        println!("It was a tie");
    }
    if user_dice < computer_dice {
        println!("You lost, Shame!!!!!!");
    }
}
fn main() {
    println!("Play a Dice game with a computer!");
    let mut count = 0;
    loop {

        if count == 3 {
            break;
        }
        let mut roll = String::new();
        println!("Do you want to roll your dice? Y/N");
         get_input(&mut roll);
         let roll = roll.trim().parse().expect("Please enter a char");

          match roll {
              'y'|'Y' => start_game(),
              'n'|'N'=> {
                        println!("Please click (y or Y) to roll a dice");
                            continue;
                        },
                _ => {
                        println!("Wrong input, enter (y or Y) to roll a dice");
                        continue;
                    }
          }
    
        count += 1;
    }   

}
