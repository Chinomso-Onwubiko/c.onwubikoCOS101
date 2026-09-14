//Program to count numbers with rust using a for loop.
use std::io;

fn main() {
    let mut lower_bound = String::new();
    println!("What number should the count start from?");
    io::stdin()
        .read_line(&mut lower_bound)
        .expect("Your input could not be stored for the lower bound, please input a valid string.");
    let lower_bound: i16 = lower_bound.trim().parse().expect("Please input an integer, not a letter/word");


    let mut upper_bound = String::new();
    println!("What number should the count stop at? Increase the number by 1 i.e to stop at 5, input 6");
    io::stdin()
        .read_line(&mut upper_bound)
        .expect("Your inputed value could not be stored.");
    let upper_bound: i16 = upper_bound.trim().parse().expect("Please input a positive whole number");


    for seconds in lower_bound..upper_bound{
        println!("Count level is: {}", seconds);
    }
}
