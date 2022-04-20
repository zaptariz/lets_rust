/* Interested guessing game */

use rand::Rng;                                                        //To generate a random numbers 
mod game;
mod scalar_and_compound;                                                             // another file game.rs called by game
mod structure;
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
    // println!(" this is random number system generated : {}",a);
    game::start_game(a);                                            //  Random number passing to start a game 
    // scalar_and_compound::data_types();                              //  Calling the data types
    // structure::structure();   
}
