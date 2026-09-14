//Using the "while" loop

use std::io;

fn main() {
    let mut numb = String::new();
    println!("Enter a number.");
    io::stdin()
        .read_line(&mut numb)
        .expect("Your input could not be stored, please enter a valid string.");
    let mut numb:i32 = numb.trim().parse().expect("Please input a valid number, it can be positive or negative, but it must be in numeric form.");


//The while loop takes a condition, and will keep on looping until the condition is no longer met, it can also be made to keep looping indefinitely
//by using it with the 'true' boolean value i.e "while true" this will give it the same use as the "loop" keyword, which would loop indefinitely 
//unless a clause is given within the loop and is met(at this point it will end the loop), but if no clause is given, then the loop will never end.
    while numb < 10 { //This will execute provided the number inputed by the use is less than 10, if the number is greater than 10, the loop would
        //not run.
        numb += 1; //This is a reassignment, not a shadowing, so the value would have an effect(become the new value of numb) outside this scope as well.
        println!("This is the current value of numb inside the while loop scope: {}", numb);
    }
    println!("This is the value of numb outside the while loop scope i.e after it has ended: {}", {numb});
}
