use std::io;
use rand::Rng;

fn main() {
    println!("guess your number bro!");
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is :{secret}");
     
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("it failed to read line");
    println!("your guess: {guess}");

    println!("game ending");
}