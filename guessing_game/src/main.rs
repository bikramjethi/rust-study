use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Developing a guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user's guess");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("User guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
