use std::io;

fn main() {
    //These tuples will server as the menu.
    let poundo: (u16, &str, f32) = (1, "Poundo Yam/Edinkaiko Soup", 3200.0);
    let f_rice: (u16, &str, f32) = (2, "Fried Rice & Chicken", 3000.0);
    let amala: (u16, &str, f32) = (3, "Amala and Ewedu Soup", 2500.0);
    let eba: (u16, &str, f32) = (4, "Eba and Egusi Soup", 2000.0);
    let r_stew: (u16, &str, f32) = (5, "White Rice & Stew", 2500.0);
    let mut total: f32 = 0.0;
    let mut order_list: Vec<(&str, f32, f32)> = Vec::new();
    let mut name = "".to_string();

    println!("Hi there, Welcome to'A tongue's Paradise', what would you like to be addressed as?");
    io::stdin()
        .read_line(&mut name)
        .expect("Please enter a valid string for your name.");
  
    loop{
        println!("");
        println!("Hi {}, here is our menu:
        (meal code, meal, price)
        {:?}
        {:?}
        {:?}
        {:?}
        {:?}
        If you are not interested in eating or buying any more food, please type 'done'...
        what would you like to eat?", name.trim(), poundo, f_rice, amala, eba, r_stew); 

        let mut choice = String::new();//This creates a new empty string object.

        println!("Please type in the meal code for the meal you want to buy into the response box: ");
        io::stdin() //This code block will take input from the user and assign it the the variable that was passed as the argument.
            .read_line(&mut choice)
            .expect("Your input could not be processed, please enter a valid string.");

        //It will check to see if the user typed in 'done', if yes, it will break the loop, and end the program.
        if choice.trim().to_lowercase() == "done" { //This will check to see if the user left the menu, or proceeded to continue with making orders.
            println!("Thank you for coming, {}, we hope to serve you again.\n", name.trim());
            break;
        }

        let choice: f32 = choice.trim().parse().expect("Please ensure you typed in the meal code as your input.");

        //I decided to add two different ways of exiting for the user, either they type 'done' or 6, whichever they are more comfortable with.
        /*if choice == 6.0 { //This will check to see if the user left the menu, or proceeded to continue with making orders.
            println!("Thank you for coming, we hope to serve you again.");
            break;
        }*/

        let mut quantity = String::new();
        println!("How many portions of the meal would you like to buy?");
        io::stdin()
            .read_line(&mut quantity)
            .expect("The value you inputed for quantity is not a valid string.");
        let quantity: f32 = quantity.trim().parse().expect("Please give a positive integer as your input for quantity.");

        //This used tuple assignments to assign values to two different variables at the same time.
        let (price, food) = match choice { //This will use the match feature to link food prices and the food bought together, based on the code the
            //user gave as input.
            1.0 => (poundo.2  * quantity, poundo.1),
            2.0 => (f_rice.2 * quantity, f_rice.1),
            3.0 => (amala.2  * quantity, amala.1),
            4.0 => (eba.2 * quantity, eba.1),
            5.0 => (r_stew.2 * quantity, r_stew.1),
            _ => {
                println!("Please give a valid input.");
                break
            }
        };
        order_list.push((food, price, quantity)); //This will append a set of tuples containing the aforemention variables to the list which was created
        //using the variable order_list.
        //order_list.push_str(" | ");
        println!("The price of {:?} is: {}",food, price);
        total += price;
        println!("Here are the items you have in your food cart, alongside their total price: {:?}, N{}", order_list, total);
    }
    //let order_list = order_list.split("|");

    for (f, p, q) in &order_list{ //This would loop through the list of tuples and print out each ordered item, alongside its portion and total price.
        println!("{q} portions of {f}: {p}")
    }

    if total > 10000.0{
        total = total * 0.95;
        let initial = (100.0/95.0) * &total;
        println!("The actual cost of your purchase was N{initial}, but since you spent above N10000, you get a 5% discount, here is the new total: N{total}");
    }
    else {
        println!("Here is the total: {total}");
    } 
    println!("Do enjoy the rest of your day, cheers!");
}

