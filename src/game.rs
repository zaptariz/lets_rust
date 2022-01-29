use colored::*;
use std::io;
pub fn start_game(a: i32) {
    loop {
        println!("Guess your number and  enter it : \n");
        let mut guess = String::new();                  //  declared a new mutable String variable guess
        io::stdin()                                      //  input stream
            .read_line(&mut guess)      //  Get the input from the player
            .expect("failed to read a number");            //  here you need to handle 
        let guess: i32 = match guess.trim().parse() {          //error handling
            Ok(num) => num,
            Err(_) => {
                println!(" try again with integers : ");
                continue;
            }
        };
        if a < guess {
            println!("{}", "your guess is big".yellow());
        } else if a > guess {
            println!("{}", "your guess is low".red());
        } else if a == guess {
            println!("{}", "you win".green());
            break;
        }
    }
}