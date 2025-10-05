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
   .expect("it failed to read line");//read_line returns a Result type //result is enum that means its a type which can be possinly in more than one or multiple states
   //result has error handeling info 
   //it expects if err throw error message and ok display value
   //if no expect then it will cmpile but u get waring
   println!("your guess: {guess}");
}