use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hi there, welcome to the guessing game, you would have to guess a secret number");
    let mut user_guess = String::new();
    let secret_numb = rand::thread_rng().gen_range(1..=100);

    io::stdin() //This takes in input from the user.
        .read_line(&mut user_guess) //This assigns the users input to the variable "user_guess" 
        .expect("Your input could not be stored.");

    let user_guess: u32 = user_guess.trim().parse().expect("Please input a number.");
    //The line of code above can also be written like this:
    //let user_guess: u32 = user_guess
      //  .trim()
      //  .parse()
      //  .expect("Please input an integer!");
        
    match user_guess.cmp(&secret_numb){
        Ordering::Less => println!("Your guess was less than the secret number, which was: {secret_numb}"),
        Ordering::Greater => println!("Your guess was greater than the secret number, which was: {secret_numb}"),
        Ordering::Equal => println!("Your guess was correct, the secret number is {secret_numb}, congratulations, you are our winner"),
    }



}