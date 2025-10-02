use std::io;

fn main() {
    println!("Developing a guessing game!");
    println!("Please input your guess:");

    let mut guess:String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read user's guess");

    println!("User guessed: {guess}");
}
