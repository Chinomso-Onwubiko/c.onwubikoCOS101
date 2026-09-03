/*use std::io;
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
}*/

/*use std::io;
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
    The line of code above can also be written like this:
    let user_guess: u32 = user_guess
        .trim()
        .parse()
        .expect("Please input an integer!");
        
    match user_guess.cmp(&secret_numb){
        Ordering::Less => println!("Your guess was less than the secret number, which was: {secret_numb}"),
        Ordering::Greater => println!("Your guess was greater than the secret number, which was: {secret_numb}"),
        Ordering::Equal => println!("Your guess was correct, the secret number is {secret_numb}, congratulations, you are our winner"),
    }
}*/



use std::io;
use std::cmp::Ordering;
use rand::Rng;
//This version of the code adds an infinite loop and a break statement.
fn main() {
    println!("Welcome to the guessing game, you are to guess the random number.");
    loop{ //This begins an infinite loop.
        println!("Please input your guess.");
        let mut u_guess = String::new(); //This creates a new mutable variable and assigns an empty string to it as its value.
        let s_number = rand::thread_rng().gen_range(1..=100); //This generates the random number that is to be guessed.
        println!("This is the secret number(for the sake of production testing): {s_number}");

        io::stdin() //This will take in input from the user.
            .read_line(&mut u_guess) /*This appends the input received from the user to the variable "u_guess" i.e it adds whatever
            input is given to its old value.*/
            .expect("Your input could not be stored."); //This will act as a fall back in the instance where an error occurs.
        /*break;//The break statement can be used anywhere within the loop, it doesn't have to be within a "match" function, if this
        line of code is uncommented, the program will stop immediately after the user gives an input.*/
        
        let u_guess: i32 = u_guess.trim().parse().expect("Please input an integer or a decimal number.");
    /*The line above first trims(takes away whitespaces at the beginnning and end of the current value of 
    u_guess[don't forget it is still a string, afterwhich it type casts the string to the integer format, which then allows us 
    to use the "compare" method from the standard library on it, "expect" also acts as a fall back here incase anything goes wrong.*/

/*NOTE: THE NUMBER TO BE GUESSED SHOULD BE INSIDE THE PARENTHESES, WHILE THE NUMBER INPUTED BY THE USER SHOULD BE THE ONE THE "cmp"
METHOD IS CALLED ON, ELSE IT WILL MESS UP THE LOGICALITY, AND RATHER THAN CHECKING IF "u_guess" is > or < or ==  "s_number", it 
checks it the other way around and that messes up the output.*/
        match u_guess.cmp(&s_number){ //This compares the "u_guess" variable to the "s_number"
            Ordering::Greater => println!("Your number is too large, the correct number is {}, try again", s_number), //If it is greater.
            Ordering::Less => println!("Your number is too small, the correct number was {}, try again.", s_number),//if smaller
            Ordering::Equal =>{
                println!("Congratulations you guessed the right number: {}", s_number); // if it is equal.    
                break;
            } 
        }

    }

}



/*use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game, you would be required to guess the randomly generated number.");
    loop{
        println!("Guess the number...");
        let mut u_guess = String::new();
        let s_number = rand::thread_rng().gen_range(1..=1000);

        io::stdin()
            .read_line(&mut u_guess)
            .expect("Your input could not be processed");

        let u_guess: i32 = match u_guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match u_guess.cmp(&s_number){
            Ordering::Greater => println!("Your number is too big, the correct number was {s_number}, try again."),
            Ordering::Less => println!("Your number is smaller, the correct number was {s_number}, try again"),
            Ordering::Equal =>{
                println!("Congratulations! You guessed the correct number: {s_number}.");
                break;
            }
        }    

    }
}*/