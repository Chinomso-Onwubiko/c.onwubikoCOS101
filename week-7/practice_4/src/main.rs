use std::io;

fn main() {
    let mut input1 = "".to_string();
    println!("Enter the input for parameter A:");
    io::stdin()
        .read_line(&mut input1)
        .expect("Your value for input1 could not be read.");
    let a: i32 = input1.trim().parse().expect("Enter an integer as the value of input1.");

    let mut input2 = String::new();
    println!("Enter the input for parameter B:");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input2");
    let b: i32 = input2.trim().parse().expect("Enter an integer as the value for Input2.");

    //call add function with arguments.
    add(a, b);
    //let c = add(a, b);
    //println!("{:?}",c);//This would print '()' i.e a null/none value representation in rust.
}

fn add(a: i32, b: i32) {
    let sum = a + b;

    println!("Sum of A and B = {}", sum)
}
