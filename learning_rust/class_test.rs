fn main() {
	println!("This is to practice for the COS quiz tomorrow.");
	let menu = "Chinomso | Divine | Onwubiko";
	for names in menu.split(" | "){
		println!("{}", names);
	}
}