//This code file will be focused on data types, and their uses, as welll as their ranges.

fn main() {
    println!("Hello, world!");
    //SCALAR TYPES: There are four scalar types in rust; Integers, Strings, Boolean, and Characters.
    //Integers: Integers are numbers in RUST, they can be further divided into 3 classes; Signed(i), Unsigned(u), and floating point(f) numbers;

    //Signed integers: These numbers are representative of all whole numbers(positive and negative), they are declared using i"Bit size") i.e i8, 
    //i16, i32, i64, i128, isize. They can hold values of the range -2^(n-1) to 2^(n-1)-1, i.e for i8, they can hold from -128 to 127, and also 
    //hold 0.
    let x: i8 = 127;
    let y: i8 = -128;

    println!("{x}");
    println!("{y}");

    //Unsigned integers: These numbers are positive whole numbers, and zero. The type is declared using the u"bit size" format i.e u8, u16, u32, 
    //u64, u128, and usize. They can hold values within the range of 0 to (2^n)-1 i.e for u8, it can hold between 0 and 255.
    let v: u8 = 255; //Highest possible value it can hold.
    let w: u8 = 0; // lowest possible value it can hold.
    let e: usize = 45; //Assigns the bit size based on the architecture of the CPU that is running the program, i.e rust would change the bit size
    //based on the computer/machine it is running on.

    println!("{v}");
    println!("{w}");
    println!("{e}");


    //floating numbers: These are decimal numbers in Rust, they are declared using the f"bit size" format i.e f32 and f64, there is no other stable 
    //f format type, also the f type does not have the "fsize" feature, so you cannot use it.
    let a: f32 = 56.04;
    let b: f64 = 58439.32;

    println!("{a}");
    println!("{b}");

    //The boolean type, Boolean types are classified into two types, 'true' and 'false'. The boolean type is declared in Rust by using the "bool" 
    //keyword, the main way boolean values are used is with conditionals, such as an 'if' expression. It would be discussed under control flow.  
    let x: bool = true;
    let l: bool = false;

    println!("{x}");
    println!("{l}");

    //Character type: The character is the primitive alphabetic type in Rust, you declare character types by enclosing them in single quotes, rather
    //than the usual double quotes, they can only be one length(character) long, it can be an emoji, letter japanese symbol, korean symbol, e.t.c.
    let c_char: char = 'c';
    //let word_char: char = 'places';//This line will return an error.

    println!("{c_char}");
    //println!("{word_char}");
}

