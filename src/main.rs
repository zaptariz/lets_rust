/* Interested guessing game */

use rand::Rng;                                                        //To generate a random numbers 
mod game;                                                             // another file game.rs called by game

/* thread_rng() => create a thread
gen_range(low , hight) => generate a random numbers between a range
    */
fn rand_number_gen() -> i32 {
    let secret_number = rand::thread_rng().gen_range(1, 10); 
    return secret_number;
}

fn main() {
    println!(" !...Lets play a guessing game...!\n");
    let a = rand_number_gen();                                  //  Assign a random value to variable a from the rand_number_gen() function
    game::start_game(a);                                            //  Random number passing to start a game 
    
}
