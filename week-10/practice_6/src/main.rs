fn add_one(e: &mut i32) {
    *e += 1;
    println!("Inside the add_one function: {e}");

}

fn main() {
    let mut i = 3;
    add_one(&mut i); 
    println!("Inside the main function, because the value was passed by reference, the actual value of i would also be changed: {}", i);
}
