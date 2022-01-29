use colored::*;
use rand::Rng;
use std::io;
fn rand_number_gen() -> i32 {
    let secret_number = rand::thread_rng().gen_range(1, 10);
    println!("{} \n", secret_number);
    return secret_number;
}
fn start_game(a: i32) {
    loop {
        println!("Guess your number and  enter it : \n");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read a number");
        let guess: i32 = match guess.trim().parse() {
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
fn main() {
    println!(" !...Lets play a guessing game...!\n");
    let a = rand_number_gen();
    start_game(a);
}
