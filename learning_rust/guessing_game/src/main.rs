use std::io;
use std::cmp::Ordering;
use rand::Rng;

//I wrote the code off hand again to ensure I had a proper understanding of how the libraries imported work, alongside the
//the structure of the rust syntax.
fn main() {
    println!("Welcome to the wonderful guessing game! Guess the random number:");
    let mut user_guess = String::new();
    io::stdin()
        .read_line(&mut user_guess)
        .expect("Your input could not be stored");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let user_guess: u32 = user_guess.trim().parse().expect("Please input an integer");

    match user_guess.cmp(&secret_number){
        Ordering::Greater => println!("Your number is too big..."),
        Ordering::Less => println!("Your number is too small..."),
        Ordering::Equal => println!("Wonderful, you guessed the right number, it was: {user_guess}"),
    }
}


//   use std::io;
//use rand::Rng;
//use std::cmp::Ordering;

//fn main() {
//    println!("Hi there, welcome to the guessing game, you would have to guess a secret number");
//    let mut user_guess = String::new();
//    let secret_numb = rand::thread_rng().gen_range(1..=100);

//    io::stdin() //This takes in input from the user.
//        .read_line(&mut user_guess) //This assigns the users input to the variable "user_guess" 
//        .expect("Your input could not be stored.");
//
//    let user_guess: u32 = user_guess.trim().parse().expect("Please input a number.");
    //The line of code above can also be written like this:
    //let user_guess: u32 = user_guess
      //  .trim()
      //  .parse()
      //  .expect("Please input an integer!");
        
//    match user_guess.cmp(&secret_numb){
//        Ordering::Less => println!("Your guess was less than the secret number, which was: {secret_numb}"),
//        Ordering::Greater => println!("Your guess was greater than the secret number, which was: {secret_numb}"),
//        Ordering::Equal => println!("Your guess was correct, the secret number is {secret_numb}, congratulations, you are our winner"),
//    }
//}