//Rust uses underscores to separate large numbers, just to ease readability.

fn main() {
    let float_with_separator = 11_000.555_001;
    println!("Float value: {}", float_with_separator);

    let int_with_separator = 50_000;
    println!("int value: {}", int_with_separator);
}
