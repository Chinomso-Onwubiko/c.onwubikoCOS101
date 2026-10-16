use std::io;

fn value(n: Option<&char>){
    println!("Element of vector: {:?}", n);
}

fn main() {

    let v = vec!['R', 'U', 'S', 'T', 'A', 'C', 'I','A', ' ', 'N'];

    let mut input1 = String::new();
    println!("\nEnter an index value between (0-8)");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");

    //index is the non negative value which is smaller that the size of the vector.
    let index: usize = input1.trim().parse().expect("Invalid input");

    //Getting value at the given index value
    let ch: Option<&char> = v.get(index);
    value(ch);
}

