use std::io;
//Writing a program to output name and age.

fn main() {
    println!("Student Information Management System");

    //input name
    let mut name = String::new();
    println!("Please enter your name.");

    io::stdin()
        .read_line(&mut name)
        .expect("Your input could not be stored.");

    /*let name = name.trim().parse().expect("");{
        Ok(age) => age,
        Err(_) => println!("Please input a whole number as your age."),
    };*/

    println!("Your name is: {name}");

    //input age
    let mut age = String::new();
    println!("\nPlease enter your age.");

    io::stdin()
        .read_line(&mut age)
        .expect("Your input for the age could not be stored");
    
    let age: u16 = age.trim().parse().expect("Please input a positive whole number as the value for your age"); 
        /*Ok(age) => age, //This block would be what would have made up the match expression if we had used it, but after thinking it through, it
        //seemed like an overkill.
        Err(_)  => {
            println!("Please input a positive whole number as the value for your age.");
            0 //The return value's data type for both arms must be the same, so this passes a return value of zero to ensure the "match" 
            //expression allows the program to crash 'gracefully'
        }*/
    //};closing delimeter for the would be 'match' block.

    println!("Your age is: {age}");
}
