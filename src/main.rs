use std::io;

fn main() {
    println!(" !...Lets play a guessing game...!");
    println!("Guess your number and  enter it : ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read a number");
    println!(" your guessing number {}", guess);
}
