use std::io::{self, Write};

fn main() {
    //Create an empty vector "city"
    let mut city: Vec<String> = Vec::new();
    //print city vector
    println!("The city veector has elements {}", city.len());

    //push new elements into.
    let mut input1 = String::new();
    println!("How many cities do you want to enter");
    std::io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input.");
    let city_num: i32 = input1.trim().parse().expect("Invalid input");

    for count in 0..city_num {
        let mut input2 = String::new();
        print!("Enter city {}: ", count + 1);
        //io::stdout().flush().expect("Failed to flush stdout");// This prevents 
        //the program from buffering the text passed in the 'print!()' macro by flushing it and ensuring it is displayed on the terminal before the user gives input.
        std::io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read input");
        let new_city: String = input2.trim().parse().expect("Invalid input");
        city.push(new_city);

    }

    print!("Your preferred cities are: \n");
    let mut count = 1;
    //loop to iterate elements in the vector:

    for cities in city{
        println!("{} {}", count, cities);
        count += 1;
    }
}
