use std::io;
use std::process;

fn main() {
    
    let levels: Vec<&str> = vec!["APS 1-2", "APS 3-5", "APS 5-8", "KL1 8-10", "KL2 10-13", "SKS"] ;
    let info: [&str; 4] = ["Office Admin", "Academic", "Lawyer", "Teacher"];
    
    let mut name = String::new(); //This creates a new empty string object.
    println!("Hello there👋👋, welcome to the Public APS level checker service, may I know your name?");
    io::stdin() //This allows the user to give input throught the keyboard.
        .read_line(&mut name) //This appends the inputed string the empty string object that was earlier created
        .expect("Invalid string entered for name.");
    let name = name.trim(); //This removes all whitespace/newline characters from the end and beginning of the code.

    //This takes the department they work under.
    println!("What department do you work under out of the following, respond using the numbers corresponding to your department
        1. {}
        2. {}
        3. {}
        4. {}", info[0], info[1], info[2], info[3]);

    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("The department you chose could not be stored.");
    let department:usize = department.trim().parse().expect("Please ensure you chose one the options given from the available list.");
    
    //This calls the various functions for each department based on an index that has been mapped to each department.

    let role_call: usize = match department{
        1 => o_admins(),
        2 => academics(),
        3 => lawyers(),
        4 => teachers(),
        _ =>  {
            process::exit(401)
        }
    };//Each of the aforementioned functions return an integer of usize type, because any string slice must be called with an integer
    //type of usize.
    
    let level = levels[role_call-1]; //We can then infer the APS level of the user by comparing the value returned from the functions 
    //above to the indexing  of the arrays.

    println!("Dear {}, based on the position you occupy, your APS level is: {}.🤝🤝
Cheers🥂🥂.", name, level);

}

/*All the functions are designed to return integers of type usize, and these integers represent the role/position of the user in 
the specific department is for, since the number of roles under each department are even, and the number of APS levels are also
equal in order, it means the index of the role in each department is equal to the equivalent level of that role, as such we can
use the index to map each role to their repsective APS level.*/

fn o_admins() -> usize {
    let office_admin: [&str; 6] = ["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    println!("Which of these positions do you hold under the office administrative department:
        1. {}
        2. {}
        3. {}
        4. {}
        5. {}
        6. {}", office_admin[0], office_admin[1], office_admin[2], office_admin[3], office_admin[4], office_admin[5]);
    let mut position = String::new();
    io::stdin()
        .read_line(&mut position)
        .expect("Please input a valid string for position occupied under office_admin");
    let index: usize = position.trim().parse().expect("Please ensure you enter the numeric representation of your position as shown in the list");
    //The line above typecasts the inputed string to an unsigned(positive) integer of data type 'usize'
    if index < 1 || index > 6{
        println!("Invalid response, please ensure your response is a number present in the list that was displayed");
        process::exit(401)
    }
    println!("Welcome, {}", office_admin[index-1]);
    //let position: u8 = position.parse().expect("Can not be type casted to u8 from usize");
    return index
}

fn academics() -> usize {
    let academic: [&str; 6] = ["_", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    println!("Which of these positions do you hold under the academic department:
        1. {}
        2. {}
        3. {}
        4. {}
        5. {}
        6. {}", academic[0], academic[1], academic[2], academic[3], academic[4], academic[5]);

    let mut position = String::new();
    io::stdin()
        .read_line(&mut position)
        .expect("Please input a valid string for position occupied under academic department");
    let index: usize = position.trim().parse().expect("Please ensure you enter the numeric representation of your academic position as shown in the list");
    if index < 1 || index > 6{
        println!("Invalid response, please ensure your response is a number present in the list that was displayed");
        process::exit(401)
    }
    println!("Welcome, {}", academic[index-1]);

    return index
}

fn lawyers() -> usize {
    let lawyer: [&str; 6] = ["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    println!("Which of these positions do you hold under the law department:
        1. {}
        2. {}
        3. {}
        4. {}
        5. {}
        6. {}", lawyer[0], lawyer[1], lawyer[2], lawyer[3], lawyer[4], lawyer[5]);

    let mut position = String::new();
    io::stdin()
        .read_line(&mut position)
        .expect("Your input for the position you occupy as a lawyer could not be stored.");
    let index: usize = position.trim().parse().expect("Ensure you input a number that has a representation of your law pedigree in the list that was given.");
    if index < 1 || index > 6{
        println!("Invalid response, please ensure your response is a number present in the list that was displayed");
        process::exit(401)
    }
    println!("Welcome, {}", lawyer[index-1]);

    index
}

fn teachers() -> usize {
    let teacher: [&str; 6] = ["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
    println!("Which of these positions do you hold under the law department:
        1. {}
        2. {}
        3. {}
        4. {}
        5. {}
        6. {}", teacher[0], teacher[1], teacher[2], teacher[3], teacher[4], teacher[5]);

    let mut position = String::new();
    io::stdin()
        .read_line(&mut position)
        .expect("Your input for the position you occupy as a teacher could not be stored.");
    let index: usize = position.trim().parse().expect("Ensure you input a number that has a representation of your role as a teacher in the list that was given.");
    if index < 1 || index > 6{
        println!("Invalid response, please ensure your response is a number present in the list that was displayed");
        process::exit(401);
        //println!("closed")//This would not print because rust ends the program in the previous line.
    }
    println!("Welcome, {}", teacher[index-1]);

    return index
}