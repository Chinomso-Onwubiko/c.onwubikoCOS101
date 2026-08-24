use std::io;

fn main() {
	println!("Hello there, welcome to our platform, to ensure we comply with the laws governing us...");
	println!("Please input your age...");
	let mut numb = String::new();
		
	io::stdin()
	    .read_line(&mut numb)
	    .expect("Your inputed value could not be stored.");
	println!("Thank you, your age: {} has been recorded.", numb.trim());
	
}