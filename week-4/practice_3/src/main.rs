use std::io;

fn main() {
    let mut base = String::new();
    let mut height = String::new();

    println!("Please enter the value for the length of the base of the triangle.");
    io::stdin()
        .read_line(&mut base)
        .expect("Your input for the base could not be stored.");
    let base: f32 = base.trim().parse().expect("Please input a number for the base.");

    println!("Please entert the value for the height of the triangle.");
    io::stdin()
        .read_line(&mut height)
        .expect("Your input for the height could not be stored.");
    let height: f32 = height.trim().parse().expect("Please input a number for the height");

    if base > 0.0 {
        let area: f32 = (base * height) * 0.5;
        println!("The area of the triangle with a base of {base} units, and a height of {height} units is {area}.");
    }
    /*if base <= 0.0 || height <= 0.0 {
        println!("Please enter a positive number i.e a number that is greater than 0.");
    } else{
        let area: f32 = (base * height) * 0.5;
        println!("The area of the triangle with a base of {base}units and a height of {height} units is: {area}");
    }*///I commented this part out for the sake of sticking to the practice exercise.
}
