use std::io;

fn main() {
	println!("Hello there, this program is going to help you calculate your average.");

	let mut test1 = String::new();
	let mut test2 = String::new();
	let mut test3 = String::new();
	let mut name = String::new();

	println!("Please enter your full name.");
	io::stdin()
	    .read_line(&mut name)
	    .expect("The input you gave for your name could not be stored, please input a valid string.");
	let name = name;.trim();

	println!("Please enter your score in the first test...");
	io::stdin()
	    .read_line(&mut test1)
	    .expect("Your score for the first test could not be stored, please enter a valid string.");
	let test1: f32 = test1.trim().parse().expect("Please input a positive numeric value as the score of your first test.");

	println!("Please enter your score in the second test...");
	io::stdin()
	    .read_line(&mut test2)
	    .expect("Your value for the second test could not be stored, please enter a valid string.");
	let test2: f32 = test2.trim().parse().expect("Please enter a positive number as the score for your second test.");

	println!("Please enter your score in the third test...");
	io::stdin()
	    .read_line(&mut test3)
	    .expect("Your score for the third test could not be stored, please enter a valid string.");
	let test3: f32 = test3.trim().parse().expect("Please enter a positive number as the value for your third test.");

	let average = (test1 + test2 + test3)/3.0;
	//let average: u32 = average as u32; This is another method of type casting.
    

    if test1 < 0.0 || test2 < 0.0 || test3 < 0.0 || test1 > 100.0 || test2 > 100.0 || test3 > 100.0{
		println!("Dear {}, one or more of the values you inputed as the score of the test is either less than 0 or greater than 100, please input \
		your actual score between the ranges of 0 and 100, where both the upper and lower limits are included.", name.trim());
	}
	else if average < 44.0 && average > 0.0{
		println!("Dear {name}, your grade on your test with a score of {average} is F.");
	}
	else if average > 44.0 && average <= 49.0{
		println!("Dear {}. your grade on your test with a score of {average} is a D.", name.trim());
	}
	else if average > 49.0 && average <= 59.0{
		println!("Dear {}, your grade on your test with a score of {average} is a C.", name.trim());
	}
	else if average > 59.0 && average <= 69.0{
		println!("Dear {}, your grade on your test with a score of {average} is a B.", name.trim());
	}
	else if average > 70.0 && average <= 100.0{
		println!("Dear {}, your grade on the test with a score of {average} is an A.", name.trim());
	}
}