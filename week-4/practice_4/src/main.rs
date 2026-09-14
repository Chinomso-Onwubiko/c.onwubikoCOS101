use std::io;

//Rust program to determine an age pass.
fn main() {
    let mut name = String::new();
    println!("Enter your name.");
    io::stdin()
        .read_line(&mut name)
        .expect("Your input for the name could not be stored, you didn't enter a valid string.");

    let mut age = String::new();
    println!("Please enter your age.");
    io::stdin()
        .read_line(&mut age)
        .expect("Your input for your age could not be stored.");
    let age: u8 = age.trim().parse().expect("Please input a positive whole number, your age cannot be a decimal or a fraction.");

    if age >= 18 { //The program checks if the users age is above or equal to 18, if true, it runs this block, if false, it runs the else block.
        println!("Welcome to the party, {}, do have a wonderful time tonight.", name.trim());
    } else {
        println!("Oops! Looks like someone is underaged, you can't attend this party, see you in {} years, do enjoy your evening. Cheers", 18 - age);
    }
}
