use std::io;
use std::io::Write;
use std::fs::File;

fn main() {
    let mut names: Vec<String> = Vec::new();
    let mut matrics: Vec<String> = Vec::new();
    let mut depts: Vec<String> = Vec::new();
    let mut lvls: Vec<u16> = Vec::new();
    
    let mut count: u16 = 1;
    let mut numb = String::new();

    print!("Hi there, how many students would you be onboarding to the Student Management Information System: ");
    io::stdout().flush().expect("Couldn't flush out the preceding print! line.");//This ensures the users input is on the same line as the question
    //prompt that was passed into the print! macro and also ensure that the string buffering is 'flushed' i.e forced to show.

    io::stdin()
        .read_line(&mut numb)
        .expect("Your value for the number of students to you want to enroll on the platform could not be stored.");
    let numb: u16 = numb.trim().parse().expect("Please input a positive integer as the value for the number of students you would want to onboard to the portal.");

    let mut db = File::create("smis.csv").unwrap();
    db.write_all("Student Name,".as_bytes()).expect("Could not add the first column");
    db.write_all("Matric. Number,".as_bytes()).unwrap();
    db.write_all("Department,".as_bytes()).unwrap();
    db.write_all("Level\n".as_bytes()).unwrap();

    while count <= numb {
        let mut name = String::new();
        let mut matric = String::new();
        let mut dept = String::new();
        let mut lvl = String::new();

        println!("Please type in your name.");
        io::stdin()
            .read_line(&mut name)
            .expect("Could not store your input for name");
        name = name.trim().to_uppercase();

        println!("Please type in your matric number.");
        io::stdin()
            .read_line(&mut matric)
            .expect("Could not store your input for matric");
        matric = matric.trim().to_uppercase();
        if matrics.contains(&matric){
            println!("Please crosscheck your matriculation number, as it matches that of another student in the database.");
            continue
        }
        //I write the name to the csv file here just so as to ensure the student is confirmed to have a unique matriculation number, since
        //using the '.pop()' method only affects the vector, and has no effect on the already written csv values.
        db.write_all(format!("{name},").as_bytes()).unwrap();
        names.push(name);
        db.write_all(format!("{matric},").as_bytes()).unwrap();
        matrics.push(matric);

        println!("Please type in your department of study");
        io::stdin()
            .read_line(&mut dept)
            .expect("Could not store your input for name");
        dept = dept.trim().to_uppercase();
        db.write_all(format!("{dept},").as_bytes()).unwrap();
        depts.push(dept);

        println!("Please input your study level");
        io::stdin()
            .read_line(&mut lvl)
            .expect("Could not store your input for name");
        let lvl:u16 = lvl.trim().parse().expect("Ensure you inputed a positive whole number as your study level.");
        db.write_all(format!("{lvl},").as_bytes()).unwrap();
        lvls.push(lvl);

        db.write_all("\n".as_bytes()).unwrap();//This moves to the next row in the spreadsheet.

        count += 1;
    }
    
    println!(""); //This is to create a blank line in the terminal to improve readability.
    println! ("names: {:?}", names);
    println!("Matriculation Numbers: {:?}", matrics);
    println!("Departments: {:?}", depts);
    println!("Level: {:?}", lvls);
    println!(""); //This is to create a blank line in the terminal to improve readability.

    for i in 0..names.len(){
        println!("{} {} {} {}", names[i], matrics[i], depts[i], lvls[i]);
    }
}