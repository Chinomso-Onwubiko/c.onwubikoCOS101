/*
fn main() {
	let p: f64 = 520_000_000.00;
	let t: f64 = 5.0;
	let r: f64 = 10.0;

	let amount: f64 = p * (1.0 + (r/100.0)).powf(t);
	let ci: f64 = amount - p;
	//println!("{}", amount);
	println!("The compound interest on the N{} loan after a period of {} years is: {}", p, t, ci);
}*/

/*This version of the program will allow take input from the user, so this would be a program that calculates simple interest
in general.*/


use std::io; //This imports the input/output method from Rust's standard Library.

fn main() {
	println!("Hi there, welcome to the Simple Interest calculator.");
	
	println!("How much did you deposit?");
	let mut p = String::new(); //This creates a mutable but technically empty string object.
	io::stdin() //This prompts the user for input.
	    .read_line(&mut p) //This takes note of what the user gives as input, and assigns its value to the already created variable 'p'.
	    .expect("Please input a decimal number for the principal, if you want to input an integer, do so with a (.0) at the end."); 
	    //The line above displays an error message incase the program fails due to the user inputing the wrong data type.
	let p: f64 = p.trim().parse().expect("Please input a decimal number.");/*Rust doesn't allow for direct referencing of floats using
	the ".read_line()" method, so I have to take in the float as a string first, then convert(type cast) it to a floating point number
	using the line above.*/

	//The remaining blocks work on the same logic.

	println!("How long is the money going to be kept for?");
	let mut t = String::new();
	io::stdin()
	    .read_line(&mut t)
	    .expect("Your input could not be stored.");
    let t: f64 = t.trim().parse().expect("Please input a decimal numebr for the time, if you want to input an integer, do so with a (.0) at the end.");
	
	println!("What annual rate would the interest be calculated in?");
	let mut r = String::new();
	io::stdin()
	    .read_line(&mut r)
	    .expect("Your input for the rate could not be stored.");
	let r: f64 = r.trim().parse().expect("Please input a decimal number for the rate, if you want to input an integer, do so with a (.0) at the end.");
	let s_interest: f64 = (p * r * t)/100.0; /*This calculates and stores the value of the simple interest based on the values given
	for the respective parameters.*/
	println!("The simple interest on N{}, at a rate of {}% for {} years is: {}", p, r, t, s_interest);
}