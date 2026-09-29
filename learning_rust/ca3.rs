use std::io;

fn main() {
	/*loop {
		let mut exit = String::new();
		println!("If you do not want to calculate any other customer's compound interest, type no.");
		io::stdin()
		    .read_line(&mut exit)
		    .expect("Your input for quitting could not be stored.");
		let exit = exit.trim().to_lowercase();
		if exit == "n"{
			break2
		}                                  
		/*let exit = match exit.trim() {
			"no" => {
				println!("OK, thank you for using the compound interest calculator.");
				break;
			}
			other => other,	
		};*/

		let mut p = String::new();
		let mut r = String::new();
		let mut t = String::new();

		println!("Enter the principal amount that was invested: ");
		io::stdin()
		    .read_line(&mut p)
		    .expect("Your inputed value for the principal could not be stored.");
		let p: f64 = p.trim().parse().expect("Please input a valid string.");

		println!("Enter the rate at which the principal was deposited.");
		io::stdin()
		    .read_line(&mut r)
		    .expect("Your input for the rate was not a valid string.");
		let r: f64 = r.trim().parse().expect("Please input a positive number.");
		
		println!("Enter the number of years the principal was deposited for.");
		io::stdin()
		    .read_line(&mut t)
		    .expect("Your input for the rate could not be stored, please enter a valid string.");
		let t: f64 = t.trim().parse().expect("The value you entered for the time is invalid, please input a numeric value.");

		let total: f64 = p * (1.0 + r/100.0).powf(t);
		let c_int: f64 = (total - p).round();

		println!("The compound interest on the principal of {p} is: {c_int}, the total amount it has grown to is: {total}.");
	};

}*/

let mut a = String::new();
io::stdin()
    .read_line(&mut a)
    .expect("Invalid string.");
let a: u32 = a.trim().parse().expect("Please input a valid number.");
if a < 100 {
	println!("a");
}
else if a < 80 {
	println!("b");
}
}
