use std::fs;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use std::io::Read;

fn main() {
    //This creates the file we plan on modifying.
    let mut file = File::create("data.txt").expect("Couldn't create");
    file.write_all("Hello there, how has your day been?".as_bytes()).expect("Couln't write the first line.");
    file.write_all("\nWelcome to my practice_4 of week-9".as_bytes()).expect("Couldn't write the second line.");
   
    //This will read the file and print out the contents before we append new content to it.
    let mut content = String::new();
    //There is no need of using the let keyword in the text below, since this is a reassignment.
    file = File::open("data.txt").expect("Could not open the given file.");
    file.read_to_string(&mut content).unwrap();
    println!("Here are the contents of the file that was created:\n {}.", content);
    println!(" "); //This is to create a blank line in the terminal in order to separate the results of each block of code.

    //This will modify the contents of the file by adding another line of text to it.
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("Cannot open the file to append to append.");
    file.write_all("This is the new line that was appended to the old text files".as_bytes()).expect("Could not append the first line.");
    file.write_all("\n Here is another line of appended text".as_bytes()).expect("Could not append the second sentence.");
    
    let mut file = File::open("data.txt").unwrap();

    let mut content2 = String::new();
    file.read_to_string(&mut content2).expect("Could not convert the text file to string for printing.");
    println!("{}", content2);
       
}
