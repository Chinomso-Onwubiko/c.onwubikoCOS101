fn main() {
    //initialization of tuple with data type
    let datatype_tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("declared_datatype_tuple contents: {:?}", datatype_tuple);

    //intialization of tuple without data type
    let no_datatype_tuple = ("Rust", "Fun", 100);
    println!("no_datatype_tuple contents: {:?}", no_datatype_tuple);

    //accessing tuple element at index 0
    println!("Value at Index 0 = {}", datatype_tuple.0);

    //accessing tuple element at index 1
    println!("Value at Index 1 = {}", datatype_tuple.1);

    //accessing tuple element at index 2
    println!("Value at Index 2 = {}", datatype_tuple.2);
}
