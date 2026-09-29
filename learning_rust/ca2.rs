use std::io;

fn main() {
	println!("Welcome to the Computer Store.");
	let laptop: (&str, &str, u64) = ("L", "Laptop", 550_000);
	let monitor: (&str, &str, u64) = ("M", "Monitor", 120_000);
	let keyboard: (&str, &str, u64) = ("K", "Keyboard", 15_000);
	let headset: (&str, &str, u64) = ("H", "Heaset", 12_000);

	println!("Where each row is in the order: (product code, product, price)
	{:?} 
	{:?} 
	{:?}
	{:?} 
	What product would you like to buy, input the product code to let us know: ", laptop, monitor, keyboard, headset);

	let mut user_choice = String::new();
	io::stdin()
	    .read_line(&mut user_choice)
	    .expect("The values you inputed could not be stored, please input a valid string.");
	user_choice = user_choice.trim().to_uppercase();

	let mut quantity = String::new();
	io::stdin()
	    .read_line(&mut quantity)
	    .expect("The value you inputed for quantity could not be stored, please enter a valid string.");
	let quantity: u64 = quantity.trim().parse().expect("Please enter a positive whole number as the quantity of items.");

	if user_choice == "M"{
		let total: u64 = monitor.2 * quantity;
		println!("The total costs of {quantity} monitor(s) is N{total}.");
	}
	else if user_choice == "L"{
		let total: u64 = laptop.2 * quantity;
		println!("The total cost of {quantity} Laptops is: N{total}.");
	}
	else if user_choice == "K"{
		let total: u64 = keyboard.2 * quantity;
		println!("The total cost of {quantity} keyboards is: N{total}.");
	}
	else if user_choice == "H"{
		let total: u64 = headset.2 * quantity;
		println!("The total cost of {quantity} headset(s) is: N{total}.");
	}
	else{
		println!("Your input doesn't match any of our product codes, please input a product code available in the menu which was displayed.");
	}
}