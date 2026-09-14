//Area of the triangle from the length of its three sides.

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Please enter the length of the first side of the triangle.");
    io::stdin()
        .read_line(&mut input1)
        .expect("Unable to store the input for the side 1 of the triangle");
    let a: f32 = input1.trim().parse().expect("Please input a positive number as the length of the 1st side of the triangle.");

    println!("Please enter the length of the second side of the triangle.");
    io::stdin()
        .read_line(&mut input2)
        .expect("Unable to store the input for side2 of the triangle");
    let b: f32 = input2.trim().parse().expect("Please enter a positive number as the length of the 2nd side of the triangle.");

    println!("Please enter the length of the third side of the triangle.");
    io::stdin()
        .read_line(&mut input3)
        .expect("Unable to store the input for the 3rd side of the triangle");
    let c: f32 = input3.trim().parse().expect("Please input a positive number as the length of the 3rd side of the triangle.");

    let s:f32 = (a + b + c)/2.0; //Half of the sum of the three sides.
    let step1: f32 = s*(s-a)*(s-b)*(s-c);
    let area: f32 = step1.sqrt();

    println!("The area of the triangle with the given sides: {}, {}, {} is {area}.", a, b, c);

}
