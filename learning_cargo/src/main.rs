use std::io;

fn main() {
    println!("Welcome, may I know your first name?");

    let my_name = "Chinomso";
    let mut fname = String::new();
    io::stdin()
        .read_line(&mut fname)
        .expect("The user gave no input");

    println!("What's your surname?");
    let mut sname = String::new();
    io::stdin()
        .read_line(&mut sname)
        .expect("The second user's input could not be stored.");

    println!("Hello {} {}.I'm {}, it's nice to meet you.", fname.trim(), sname.trim(), my_name.trim());
    welcome()
}

fn welcome() {
    println!("Welcome to another function");
}
