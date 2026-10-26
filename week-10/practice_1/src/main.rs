fn main() {
    let v = vec![101, 250, 330, 400];
    //vector v owns the object in heap

    //only a single variable owns the heap memory at any given time
    let v2 = v;
    //here two variable own heap value, 
    //two pointers to teh same content is not allowed in rust

    //Rust is very smart in terms of memory access, so it detects a race
    //condition if two variabes point to the same heap.
    println!("{:?}", v2);
    //println!("{:?}", v);//this will print an error.

}
