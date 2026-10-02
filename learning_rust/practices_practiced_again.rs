fn main() {
    /*let name1 = "Ayomide Adesokan";
    println!("My name is {}", name1);

    //find and replace 
    let name2 = name1.replace("Ayomide", "Adebare");
    println!("You can also call me {}", name2);
    let faculty = "Faculty of Science and Technology";

    //find and replace 
    let school = faculty.replace("Faculty", "School");
    println!("I am a student of the {} school.", school);*/

    //practice_4
    /*let fullname = "Chibudum John Umeh";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";

    let mut school = "School of Science".to_string();
    //push string
    school.push_str(" and Technology");

    println!("My name is: {}", fullname);   
    //checking the length of the fullname.
    println!("The length of my fullname is {}", fullname.len());
    println!("I am a student of {} Department", department);
    println!("{}", school);
    println!("{}", uni);*/

    //practice_5
    /*let fullname = " Pan-Atlantic University ";
    println!();
    println!("Name: {}", fullname);
    println!("");
    println!("Before trim ");
    println!("Length is {}", fullname.len());
    println!("");
    println!("After trim ");
    println!("Length is {}", fullname.trim().len());
    */

    //practice_6
    /*let n1 = "Electrical".to_string();
    let n2 = "Electronic".to_string();
    let n3 = "Engineering".to_string();
    let n4 = n1 + &n2 + &n3; //n2 and n3 reference is passed.

    //About Electrical/Electronic Engineering:
    println!("\nThe {} is informed by the aspiration to train
        electrical/electronic engineering professionals in 
        the areas of design, building and maintenance of 
        electrical control systems.", n4);

    let w1 = "Computer".to_string();
    let w2 = " Science".to_string();
    let w3 = w1 + &w2; //w2 reference is passed.
    println!();
    println!("{} is aimed at developing competent, creative,
        innovative, entrepreneurial and ethically-minded persons,
        capable of creating value in the diverse fields of {}.", w3, w3);
    */

    //practice_7: The 'format!' macro:
    /*let k1 = "Yemisi".to_string();
    let k2 = String::from(" Shyllon");
    let k3 = String::from(" Musuem");
    let k4 = " of".to_string();
    let k5 = " Art".to_string();
    let k6 = " PAU".to_string();

    //format macro use:
    let k7 = format!("{} {} {} {} {} {}", k1, k2, k3, k4, k5, k6);
    //print output:
    println!("{}", k7);
*/
    
    //practice_8: Arithmetic
    /*let num1 = 10;
    let num2 = 2;
    let mut result: i32; Rust allows for variable type declaration
    for an uninitialized variable, but only if the compiler can
    confirm that the variable will only be made reference to after
    it has been assigned a value within the code i.e if we were to
    have the "println!" line before the first "result" variable
    assignment, rust would have thrown up an error.*/

    /*result = num1 + num2;
    println!("Sum: {result}");

    result = num1 - num2;
    println!("Difference: {result}");

    result = num1 * num2;
    println!("Product: {result}");

    result = num1/num2;
    println!("Quotient: {result}");

    result = num1 % num2;
    println!("Modulus: {}", result);*/

    //practice_9: Relational operators...
    //They compare two values and return a bool:
    /*let A:i32 = 10;
    let B:i32 = 20;

    println!("Value of A: {}", A);
    println!("Value of B: {}", B);

    let mut res = A > B;
    println!("A is greater than B: {}", res);

    res = A < B;
    println!("A is less than B: {}", res);

    res = A >= B;
    println!("A is greater than or equal to B: {}", res);

    res = A <= B;
    println!("A is less than or equal to B: {}", res);

    res = A == B;
    println!("A is equal to B: {}", res);

    res = A != B;
    println!("A is not equal to B: {res}");*/

    //Practice_10: Logical Operators...
    //They are similar to relational operators, but they combine 
    //conditions like logic gates.

    let a = 20;
    let b = 30;

    if (a>10) && (b>10) { //And: both must be true to return true.
        println!("true");
    }

    let c = 0;
    let d = 30;

    if c > 10 || d > 10 { //Or: either must be true to return true.
        println!("true");
    }
    let is_elder = false;

    if !is_elder { //is_elder is False, hence !false = true, so the
    //code block will run.
        println!("Not Elder");
    }
}