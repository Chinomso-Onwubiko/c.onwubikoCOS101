//Rust has immutable variable values by default, but it allows you to mutate those variables if you defined them with the keyword "mut".
//NOTE: Variable reassignment is different from shadowing.
fn main() {
    let mut fees = 25_000;
    println!("fees is: {}", fees);

    fees = 35_000;
    println!("fees changed to: {}", fees);
}
