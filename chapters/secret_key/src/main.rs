use std::io;
use rand::Rng;

fun main(){
    println("guess your number bro!");
    let secret = rand::thread::rng().gen_range(1..=100);
    println!("the secret number is :{secret}");
     
    let mut guess = string::new();

    io::stdin().readline(&mut guess)
    .expect("it failed to read line");
    println!("your guess: {guess}");ß

}