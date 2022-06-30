use std::io;
use rand::Rng;

#[derive(Debug)]
enum Winner {
    USER,
    COMPUTER,
    DRAW
}
#[derive(Debug)]
struct Result {
    user_dice:u8,
    computer_dice:u8,
    winner:Winner
}




fn get_input( input: &mut String)  {
    io::stdin()
        .read_line(input)
            .expect("Please enter something");
}

fn roll_dice() -> u8 {
    
    let dice = rand::thread_rng().gen_range(1..6);
    dice
}

fn start_game() ->(u8, u8, Winner) {
        
    let user_dice = roll_dice();
    let computer_dice = roll_dice();
    println!("You rolled {}, Computer rolled {}", user_dice, computer_dice);
    if user_dice > computer_dice {
        println!("You won!");
        (user_dice, computer_dice, Winner::USER)
    } else if user_dice == computer_dice {
        println!("It was a tie");
        (user_dice, computer_dice, Winner::DRAW)
    }else  {
        println!("You lost, Shame!!!!!!");
        (user_dice, computer_dice, Winner::COMPUTER)
    }
}
impl Result {
    fn display_final_result(result: &Vec<Result>) {
        let mut user_final_result :u8 = 0;
        let mut computer_final_result :u8 = 0;
        let mut draws: u8 = 0;
        for (_i, res) in result.iter().enumerate() {
            match &res.winner {
                Winner::USER=> { user_final_result += 1},
                Winner::COMPUTER=>{computer_final_result += 1},
                _=>(draws += 1)
            }
        }
        println!("You have {} wins, the Computer won {} times, and there were {} Draws", user_final_result, computer_final_result, draws);
    }
}


fn main() {
    println!("Play a Dice game with a computer!");
    let mut count = 0;
    let mut result_array:Vec<Result> = Vec::new();

    loop {

        if count == 3 {
            break;
        }
        let mut roll = String::new();
        println!("Do you want to roll your dice? Y/N");
         get_input(&mut roll);
         let roll = roll.trim().parse().expect("Please enter a char");

          match roll {
              'y'|'Y' => {
                            let (user, computer, roll_result) = start_game();
                            let roll_result = match roll_result {
                                Winner::USER => Winner::USER,
                                Winner::COMPUTER=>Winner::COMPUTER,
                                Winner::DRAW=>Winner::DRAW
                            };
                            result_array.push(Result 
                                                    { 
                                                        user_dice: user, 
                                                        computer_dice: computer, 
                                                        winner:  roll_result
                                                    }
                                                )        
                        },
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

    println!("End game.");
    Result::display_final_result(&result_array);

}
