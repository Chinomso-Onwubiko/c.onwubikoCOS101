//Using the 'continue' keyword in rust;

//This program will count the amount of numbers greater than 10 between 1 and 20.
fn main() {
    let mut count = 0;

    for numb in 1..21{
        if numb <= 10{
            println!("This number is going to be skipped because it is less than 10: {:?}", numb);
            continue
        }
        count += 1;
    }

    println!("There are {count} numbers between 1 and 20 that are greater than 10"); //The output of count at this point should be 10.
}
