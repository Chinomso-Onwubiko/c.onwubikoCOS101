//use std::io;

fn main() {
	let p: f64 = 520_000_000.00;
	let t: f64 = 5.0;
	let r: f64 = 10.0;

	let amount: f64 = p * (1.0 + (r/100.0)).powf(t);
	let ci: f64 = amount - p;
	println!("{}", amount);
	println!("The compound interest on the N{} loan after a period of {} years is: {}", p, t, ci);
}