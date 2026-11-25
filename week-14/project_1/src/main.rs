use std::fs::File;
use std::io::Read;
use std::io;


fn admin() -> String{
    let mut admin_file = File::open("globacom_dbase.sql").expect("Could not open the Admin access level database.");
    let mut contents = String::new();
    admin_file.read_to_string(&mut contents).expect("The admin database file could not be written to the string object.");
    
    contents
}

fn project_m() -> String{
    let mut projects = File::open("project.sql").expect("Could not open the project table in the database.");
    let mut contents = String::new();
    projects.read_to_string(&mut contents).expect("Could not write the project table to a string object");

    contents
}

fn employee() -> String {
    let mut staffs = File::open("staff_tb.sql").expect("Could not open the staff table in the database.");
    let mut contents  = String::new();
    staffs.read_to_string(&mut contents).expect("The staff table could not be written to a string object.");

    contents
}

fn customer() -> String {
    let mut customers = File::open("customer.sql").expect("Could not open the customer table in the database");
    let mut contents = String::new();
    customers.read_to_string(&mut contents).expect("The table of customers could not be written to a string object.");

    contents
}

fn vendor() -> String{
    let mut vendors = File::open("dataplan.sql").expect("The table of dataplans could not be opened.");
    let mut contents = String::new();
    vendors.read_to_string(&mut contents).expect("The table of dataplans in the database could not be written to a string object");

    contents
}
fn main() {
    let mut input = String::new();
    println!("Hi there, welcome to Globacom Schema, which of these categories do you fall under?
        1. Administrator
        2. Project Manager
        3. Employee
        4. customer
        5. Vendor
        Please input the number that matches your user status.");

    io::stdin()
        .read_line(&mut input)
        .expect("Your input could not be parsed, please enter a valid string.");
    let status: u8 = input.trim().parse().expect("Your input could not be stored, ensure you entered a valid integer between 1 and 5, where both numbers are inclusive.");

    let access = match status {
        1 => admin(),
        2 => project_m(),
        3 => employee(),
        4 => customer(),
        5 => vendor(),
        _ => "You don't have access to the company's database.".to_string()
    };

    println!("{access}")
}