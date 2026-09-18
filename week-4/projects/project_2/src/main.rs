//This program will ask an employee whether or not he has experience in his field of work, and would also ask for his/her age, then would determine 
//the employee's annual incentive from the information that was given.
use std::io;

fn main() {
    println!("Hello our very special employee! We would like to offer annual bonuses for your good work.");

    let mut age = String::new();
    let mut exp = String::new();

    println!("Enter your age.");
    io::stdin() //This line takes input from the user(every occurence of it).
        .read_line(&mut age)
        .expect("Your input for your age could not be stored, please input a valid string.");
    let age: u8 = age.trim().parse().expect("Please enter a positive whole number as your age, don't try to enter it in decimal or words.");

    //This code block askes the user whether or not he/she is experienced, this will enable the program know which employee has experience.
    println!("Are you experienced? Enter yes or no?");
    io::stdin()
        .read_line(&mut exp)
        .expect("The value you inputed for your experience could not be stored, please input a valid string.");
    let exp = exp.trim().to_lowercase(); //This is to remove whitespaces from the beginning and the end of the users input.
    //while the '.to_lowercase()' is to convert the users input to small letters, that way even if they input uppercase letters, rust would be
    //a

    //The block below checks to see which of the criteria the employee matches, and prints out their deserved annual incentive based off of their input.
    if exp == "yes" && age >= 40{
        println!("Your annual incentive based on your experience and age would be: N1,560,000");
    }
    else if exp == "yes" && age <= 39 && age >= 29{
        println!("Your annual incentive based on your experience and age would be: N1,480,000");
    }
    else if exp == "yes" && age <= 28 {
        println!("Your annual incentive based on your experience and age is N1,300,000.");
    }
    else if exp == "no" {
        println!("Your annual incentive based on your experience level is N100,000.");
    }
    else if exp != "yes" && exp != "no" {
        println!("Please ensure you inputed 'yes' or 'no'.");
    }
    else{ //This will handle any other potential error.
        println!("You are not experienced enough to receive an annual incentive under company policy, keep working hard, one day you will get there.");
    }

}
