fn main() {
    let v = vec![20, 40, 60, 80];
    //vector v owns the object in the heap.

    let v2 = v;
    let v2_return = display(v2);
    //println!("In main: {:?}", v2); //This will not run because it lost
    //ownership of the vector to the function.

    println!("In main: {:?}", v2_return);//This will run because the value
    //gotten from the function was returned.
}

fn display(v: Vec<i32>) -> Vec<i32> {
    //returning the same vector
    println!("Inside display: {:?}", v);
    return v;
}
