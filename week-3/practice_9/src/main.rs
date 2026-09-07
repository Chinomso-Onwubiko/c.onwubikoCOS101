//I already fixed the error in the previous code, so this one is simply a repeat of that one.
//Also, I would try to differentiate between shadowing and reassignments.

fn main() {
    let mut fees = 25_000;
    println!("fees is {}:", fees);
    //fees = 35_000; //This is a reassignment, you do not need to use the 'let' keyword to perform a reassignment, you only need to have the original variable as a mutable.
    //The only time you would have needed the 'let' keyword for a reassignment would be if you wanted to perform change the original type of the variable, and in that case it becomes shadowing.
    //let fees: f64 = 233_879.8392;
    
    {
        let fees: f32 = 30_000.45; //You can shadow an immutable variable, but you can not perform a reassignment on an immutable variable.
        println!("This is the value of fees when it is shadowed in a scope: {}", fees);
    }
    println!("fees changed to: {}", fees);
}
