use std::io::Read;
use std::io::Write;

fn main() {
    let mut new = std::fs::File::create("Welcome_message.txt").expect("Create failed.");
    new.write_all("Welcome to COS101 week-9's class".as_bytes()).expect("Couldn't create the file.");
    println!("The welcome message text file has been created.");
    let mut file = std::fs::File::open("Welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
