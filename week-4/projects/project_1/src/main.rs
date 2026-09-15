//This rust program would find the roots of any given quadratic equation.

fn main() {
    println!("Where ax^2 + bx + c represents a standard quadratic equation, please input the values of a, b and c.");

    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter the value of a.");
    io::stdin()
        .read_line(&mut a)
        .expect("The value inputed for 'a' could not be stored, string not valid.");
    let a: f32 = a.trim().parse().expect("Please input a numeric value for a.");

    println!("Enter the value of b");
    io::stdin()
        .read_line(&mut b)
        .expect("The inputed value of 'b' could not be stored");
    let b: f32 = b.trim().parse().expect("Please input a numeric value for b.");

    println!("Enter the value of c");
    io::stdin()
        .read_line(&mut c)
        .expect("The inputed value for 'c' could not be stored, string not valid.");
    let c: f32 = c.trim().parse().expect("Please input a numeric value for c.");


    let determinant = b.powi(2) - 4.0 * a  * c;
    let root_1: f32 = (-b + (b.powi(2) - 4.0 * a * c).sqrt())/(2.0 * a);
    let root_2: f32 = (-b - (b.powi(2) - 4.0 * a * c).sqrt())/(2.0 * a);

    if determinant > 0.0 {
        println!("From the determinant of the equation which was deduced from the inputed values, the roots of this equation would be real and distinct.");
        println!("The roots of the quadratic equation are {root_1} and {root_2}");
    }
    else if determinant == 0.0 {
        println!("From the determinant of the equation which was deduced from the inputed values, the roots of this equation would be repeated i.e the same value \
        for both roots");//The backward slash sign used in this println! statement is a line breaker symbol, it tell the program that we are continuing the 
        //the println! statement on the next line.
        println!("The roots of the quadratic equation are {root_1} and {root_2}");
    }
    else {
        println!("From the determinant of the equation which was deduced from the given values of the equation, the roots of this equation are imaginary,\
        as such they can not be represented as real numbers.");
    }
 
}

use std::io; //I wanted to confirm if the program would still run if the crate was called at the end rather than the beginning of the code.