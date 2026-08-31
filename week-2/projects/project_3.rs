fn main() {
	let p: f64 = 210000.0;
	let r: f64 = 5.0;
	let t: f64 = 3.0;

	let amount = p * (1.0 - (r/100.0)).powf(t);
	let depre: f64 = p - amount;
	println!("The depreciation in the value of the TV set after 5 years at a rate of 5 percent is: {}", depre);
}