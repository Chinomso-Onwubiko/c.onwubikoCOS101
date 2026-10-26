fn main() {
    let v = vec![10, 20, 30];
    //vector v owns the object in the heap.

    let v2 = v; //moves ownership to v2.
    
    display(v2); //ownership of the vector is moved to the function 
    //'display()' and v2 becomes invalid.

    //println!("In main: {:?}", v2);//v2 is no longer usable here.//This will cause an error.

}

fn display(v:Vec<i32>){
    println!("Inside display: {:?}", v);
}
