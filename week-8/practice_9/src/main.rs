fn main() {

    let b:(i32, bool, f64) = (110, true, 10.9);
    print(b); /*NOTE: This is a call to the function that was written
    below, not the in-built 'print!' macro(yeah, I know it is bad 
    practice to name a function or variable like this, but this is
    just an example)*/

    //The type passed to the function must be an exact match to the
    //parameter used.

}

//pass the tuple as a parameter

fn print(x:(i32, bool, f64)) {
    println!("Inside print method");
    println!("{:?}", x);
}
