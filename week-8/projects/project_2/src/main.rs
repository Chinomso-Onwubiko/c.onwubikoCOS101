use std::io::{self, Write};

fn main() {
    let mut applicants: Vec<(u8, String)> = Vec::new();
    let mut track: u32= 0;
    let mut counts = String::new(); 
    println!("How many applications would you like to take in?");
        io::stdin()
            .read_line(&mut counts)
            .expect("Your inputed value for stop could not be stored.");
        let counts: u32 = counts.trim().parse().expect("Your inputed value for the number of applicants couldn't be stored");

    loop {
        println!("Please enter your full name:");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("The value you inputed for name could not be stored.");
        let name = name.trim().to_string();

        println!("How many years of experience do you have?");
        let mut exp = String::new();
        io::stdin()
            .read_line(&mut exp)
            .expect("The value you inputed for age could not be stored.");
        let exp: u8 = exp.trim().parse().expect("Please enter a positive integer as the value of your experience.");

        applicants.push((exp, name));
        track += 1;
    
        if track == counts { //This tells the loop when to stop(once the number of applicants taken is up to counts).
            break
        }
    } 
    let mut choice: Vec<(u8, &String)> = Vec::new();
    let mut max: u8 = 0;
    for (exp, name) in applicants.iter(){
        //println!("Max number: {}, Current Experience: {}", max, exp); //This was used to test the code, and confimr what was happening in
        //the background.
        if *exp > max {
            let dlt = choice.pop(); //removes the previously added applicants tuple and stores its wrapped value to dlt.
            //println!("{:?}", dlt);
            max = *exp; //assigns the highest value gotten to max.
            //println!("{}",max); //This was also used when checking the code for errors.
            choice.push((max, name)); //appends a tuple containing the applicants name and years of experience.
        }
    }
    
    println!("{:?}", choice);
    for (exp, name) in choice{
        println!("The applicant with the highest years of experience, and as such the best fit for the job, is {name}, with an experience of {exp} years.");
    }
            

}
