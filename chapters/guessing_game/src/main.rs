use std::io;
fn main(){
    println!("Guess the number!");
    println!("Please input your guess.");
    //mut mutable and new is assosciative function of the string instance

    let mut guess = String::new();
   // getting stdin standard input from the io module
   //if we dont import it from beginneing then we have to use std::io::stdin()
   //readline is a method that takesa mutablerefrence to stirng by
   //passing &mut guess
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}