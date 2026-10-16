fn main() {
    let b:(i32, bool, f64) = (30, true, 4.9);
    print(b);

}

fn print(x:(i32, bool, f64)) {
    println!("Inside the print method");
    //assigns a tuple to distinct variables

    let (age, is_male, cgpa) = x; //Each variable takes the value of
    //the matching elements in the tuple.
    println!("Age is {}, isMale? {}, cgpa is {}", age, is_male, cgpa);
}
