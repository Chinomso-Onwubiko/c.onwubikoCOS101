fn main() {
    // a list of numbers.
    let v = vec![15, 25, 35, 45, 55];
    let new = print_vector(v);
    //println!("{}", v[0]); //This line gives an error because v no longer 
    //owns the list of numbers, so it can't make any call to any index.

    println!("Inside main, calling the index with the returned vector:
        {:?}", new[0]); //This will print 15.
}

fn print_vector(x: Vec<i32>) -> Vec<i32> {
    println!("Inside print_vector function: {:?}", x);
    x
}