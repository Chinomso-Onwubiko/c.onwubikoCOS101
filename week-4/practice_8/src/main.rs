/*Using the "loop" keyword, the "loop" keyword is similar to the "while" keyword in rust, except the "loop" keyword takes no condition and would
loop indefinitely unless a clause is given within the loop block(the loop will end when the condition given within its block is met), but if it 
is never met, or no clause is given, it will go on without an end. It can be equated to the 'while true' statement*/

fn main() {
    println!("Welcome to the indefinite loop");

    let mut x: u8 = 0;

    loop{//while true //This will also give the same result, but rust would "warn" you to use "loop" instead.
        x += 1;
        println!("{x}");

        if x == 15 {
            break
        }
    }
}
