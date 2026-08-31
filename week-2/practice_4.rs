fn main() {
	let p: f64 = 1000.0; //Principal
	let r: f64 = 1.0; //Rate
	let t: f64 = 2.0; //Time

	//simple interest:
	let a: f64 = (p * r * t)/100.0; //Simple interest earned on the deposited sum.
	println!("The simple interest on the {} you deposited for a period of 2 years at an annual rate of 1% is: {}", p, a);

}