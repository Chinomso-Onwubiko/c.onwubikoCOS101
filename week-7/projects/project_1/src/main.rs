use std::io;
use std::process;

//This askes the user what shape he/she wants to perform a calculation for.
fn shape_selector() -> String{ 
    let shapes: [&str; 5] = ["Trapezium", "Rhombus", "Parallelogram", "Cube", "Cylinder"];
    
    println!("Please which of these shapes would you like to calculate the area for?\n");
    let shape_list = shapes.iter(); //This converts the array to an iterable.
    for shape in shape_list{
        println!("{shape}");
    }
    println!(""); //This is to create a new line in the command line and make the output more readable.

    let mut choice = String::new(); //This creates a new empty string object.
    io::stdin()
        .read_line(&mut choice)
        .expect("Your value for your choice of shape could not be stored.");
    let choice = choice.trim().to_lowercase(); //the .to_lowercase() method converts whatever
    //string type it is called on to lowercase letters.

    return choice;
}

//This askes the user to state what units the values are going to be given in.
fn unit_selector() -> String{
    let mut choice = "".to_owned(); //This converts the string literal it is called on to a string object.

    println!("Enter what unit you would be working in, e.g cm or m");
    io::stdin()
        .read_line(&mut choice)
        .expect("Your input for your unit could not be stored.");
        //you must use the expect method whenever the .parse() method is called on a value.
    //let choice: String = choice.trim().parse().expect("Please enter a valid String");
    let choice: String = choice.trim().to_string();//I used this because it is shorter.

    //The block of code below checks to see if the user gave a valid unit, if the unit isn't valid,
    //it stops the remaining code from executing.
    if choice != "mm" && choice != "cm" && choice != "m" { 
        println!("Your unit must be in millimeters(mm), centimeters(cm) or meters(m)");
        process::exit(401)
    }

    choice
}

//This defines the formula for the area of a trapezium.
fn trapezium() -> f64{
    let mut base1 = String::new();
    let mut base2 = String::new();
    let mut height = String::new();
    
    println!("Please enter the value of base1 of the trapezium.");
    io::stdin()//This takes in input from the user.
        .read_line(&mut base1) //This appends the users input to the variable passed as its argument.
        .expect("Your value for base1 could not be stored.");
    let base1: f64 = base1.trim().parse().expect("The inputed value could not be stored, ensure your input was numeric.");
    
    println!("Please input the value for base2 of the trapezium");
    io::stdin()
        .read_line(&mut base2)
        .expect("Your value for base 2could not be stored.");
    let base2: f64 = base2.trim().parse().expect("The inputed value could not be stored, ensure your input was numeric.");

    println!("Please input the value for the perpendicular height of the trapezium.");
    io::stdin()
        .read_line(&mut height)
        .expect("Your inputed value for height could not be stored.");
    let height: f64 = height.trim().parse().expect("Ensure your input for the height of the trapezium is numeric");
    
    let area: f64 = 0.5 * (base1 + base2) * height;
    area
}

//This defines the formula for calculating the volume of a cylinder.
fn cylinder() -> f64{
    let mut r = "".to_string();
    let mut height = String::new();

    println!("Please enter the radius of the cylinder.");
    io::stdin()
        .read_line(&mut r)
        .expect("Your value for the radius could not be stored.");
    let r: f64 = r.trim().parse().expect("Please ensure your input for the radius of the cylinder was a numeric value.");

    println!("Please enter the value for the height of the cylinder.");
    io::stdin()
        .read_line(&mut height)
        .expect("Your value for the height of the cylinder could not be stored.");
    let height: f64 = height.trim().parse().expect("Please ensure your value for the height of the cylinder is numeric.");
    let volume: f64 = 3.142 * r.powi(2) * height;
    return volume
}

//This defines the formula for calculating the surface area of a cube.
fn cube() -> f64 {
    let mut length = "".to_owned();
    //let mut breadth = String::from(""); //There is no need for this since cubes have all sides equal in length.

    println!("Please input the length of the side of the cube, all sides of a cube are equal in length.");
    io::stdin()
        .read_line(&mut length)
        .expect("Your value for the length of the side of the cube could not be stored.");
    let length: f64 = length.trim().parse().expect("Please ensure you input a numeric value for the lenght of the side of the cube.");
    let s_area = 6.0 * length * length;
    return s_area
}

//This defines the formula for calculating the area of a parallelogram.
fn parallelogram() -> f64 {
    let mut length = "".to_owned(); 
    let mut height = String::from(""); //This creates a string object with("from") the string literal given as its argument.

    println!("Please input the length of the longer side of the parallelogram.");
    io::stdin()
        .read_line(&mut length)
        .expect("Your value for the length of the longest side of the parallelogram could not be stored.");
    let length: f64 = length.trim().parse().expect("Please ensure you input a numeric value for the length of the longest parallegram side.");

    println!("Please input the value for the perpendicular height of the parallelogram.");
    io::stdin()
        .read_line(&mut height)
        .expect("Your inputed value for height could not be stored.");
    let height: f64 = height.trim().parse().expect("Ensure your input for the height of the parallelogram is numeric");
    let area  = length * height;
    area
}

fn rhombus() -> f64 {
    let mut diagonal1 = "".to_owned();
    let mut diagonal2 = String::from("");

    println!("Enter the length of the first diagonal of the rhombus.");
    io::stdin()
        .read_line(&mut diagonal1)
        .expect(&mut diagonal1);
    let diagonal1: f64 = diagonal1.trim().parse().expect("Please ensure your input for diagonal1 is numeric.");

    println!("Enter the length of the second diagonal of the rhombus.");
    io::stdin()
        .read_line(&mut diagonal2)
        .expect(&mut diagonal2);
    let diagonal2: f64 = diagonal2.trim().parse().expect("Please ensure your input for diagonal1 is numeric.");
    let area: f64 = (diagonal1 * diagonal2)/2.0;

    area
}

fn main() {
    //println!("{:#?}", a); //I wanted to do something different ealier, but I found a better way.
    let a = shape_selector();

    let main: f64 = match a.as_str() {
        "trapezium" => trapezium(),
        "rhombus" => rhombus(),
        "parallelogram" => parallelogram(),
        "cube" => cube(),
        "cylinder" => cylinder(),
        _ => 0.0
    };
    
    if main == 0.0 {
    println!("Invalid shape selected; please enter one of the shapes given in the list of shapes.");
    process::exit(401)
    }

    let choice = unit_selector();

    if a == "trapezium" || a == "rhombus" || a == "parallelogram" || a == "cube"{
        println!("{main}{choice}²");
    }
    else {
        println!("{main}{choice}³");
    }
}


