//Rust program to read the height of a person and print if the person is tall, dwarf or average in height.

use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your height(in centimetres):");
    io::stdin()
        .read_line(&mut input)
        .expect("Not a vlaid string");
    let height: f32 = input.trim().parse().expect("Please input you height in form of a decimal i.e if it is a whole number, add a '.0' to it.");

    if height >= 150.0 && height <= 170.0
    {
        println!("You are of average height.");
    }
    else if height > 170.0 && height <=195.0
    {
        println!("You are tall");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("You are not too tall.");
    }
    else{
        println!("Abnormal height i.e You are either too tall or too short.");
    }

}
