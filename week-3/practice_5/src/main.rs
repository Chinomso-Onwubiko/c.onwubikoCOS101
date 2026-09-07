fn main() {
    let special_character = '@'; //default type inference(rust will infer it is a character and have that as its data type.)
    let alphabet: char = 'B'; //A character is represented by enclosing the letter or alphabet in single quotation marks.
    let surname = "Bolaji";
    let first_name = "Michael";
    let middle_name = "Gboyega";

    println!(" ");
    println!("Special character: {}", special_character);
    println!("Alphabet: {}", alphabet);
    println!("Name: {} {} {}", first_name, middle_name, surname);
}
