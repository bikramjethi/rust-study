use std::io;
use rand::Rng;

fn main() {
    println!("Developing a guessing game!");
    println!("Please input your guess:");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    let mut guess:String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read user's guess");

    println!("User guessed: {guess}");
    // add a comment
}
