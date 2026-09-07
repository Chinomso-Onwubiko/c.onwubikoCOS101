fn main() {
    let result = 10.00; //Rust infers an f64 data type by default for decimal numbers.
    let interest: f32 = 8.35;
    let cost: f64 = 15000.600; //double precision

    println!("Result value is: {}", result);
    println!("Interest is: {}", interest);
    println!("Cost is {}", cost);
}
