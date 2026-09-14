fn main() {
    //"let" cannot be used to create a global variable i.e "Let" cannot be used outside a function, so their validity is limited to 
    //the scope they were created in.
    let mut x = 5; //This is how to declare a mutable variable in rust.
    println!("The value of x is: {x}");
    x = 6; //Reassignments don't need the use of the 'let' keyword, except if you would be declaring a new type for the variable
    //whose value is being modified i.e if we were to do x: i32 = 6, we would have needed to use the "let" keyword(This is called
    //shadowing, it is discussed below).
    println!("The value of x is: {x}");

    const MY_COURSE: &str = "Computer Science"; /*This is how you declare a constant in rust, the variable name should always be written in 
    upper-case and if it is more than one word, the words should be separated by underscores(it is not compulsory, but it a rust 
    programming "best practice", the code will run if it isn't done, but a warning would be displayed, stating it is not standard 
    practice not to do so.) "const" variables are encoded into the actual binary file(i.e they are compiled and every occurence 
    of the variable is replaced by its actual value in the compiled binary file. You must also declare the variables data type 
    upon its definition.*/
    const myname: &str = "Chinomso"; //This line of code would still be valid, although it doesn't keep to Rust's programming "best
    //practices".

    /*Also, a constant variable can only have a value that can be calculated or is known at compile time i.e a constants value cannot
    be an input() statement, a database import, a file import e.t.c It has to be something that can be gotten right from compile time
    e.g 2 * 2, 400, a string, e.t.c and the data type must have also been clearly stipulated.*/

    println!("This is the output of the properly named 'const' variable(according to Rust's best practices): {MY_COURSE}");
    println!("This is the output of the improperly name 'const' variable(according to Rust's best practices): {myname}");

    let x = 35; /*This is called shadowing, this is basically you reassigning a new value to a variable that already exists.
    The difference between shadowing and reassignments is the use of the 'let' keyword, if you use the 'let' keyword, the 
    new assignment only takes effect within the current scope, but if you don't use the 'let' keywor the value of the 
    reassignment even in an inner scope will have an effect on the value of x in the outer scope, which would lead to the
    permanent change in value of the variable.

    For more compact definition, the core difference between shadowing and reassignment in Rust is that shadowing creates a 
    brand-new variable with a new memory location (potentially of a completely different data type), while reassignment modifies 
    the value of an existing variable without changing its type or memory slot.*/

    println!("This is the value of x after the overshadow: {x}");
    {
        let x = 100; //If we didn't use the 'let' keyword here, the value of x in the global scope would have been changed.
        println!("This is the value of x after being shadowed inside a scope, note the value within this scope: {x}");
    };

    println!("This is the value of x outside the scope, without anymore shadowing: {x}")

}
