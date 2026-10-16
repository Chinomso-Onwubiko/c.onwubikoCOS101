use std::io;

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
       
    
        if track == counts {
            break
        }
    } 
    let mut choice: Vec<(u8, &String)> = Vec::new();
    let mut max: u8 = 0;
    for (exp, name) in applicants.iter(){
        if max == 0{
            max == *exp;
            choice.push((*exp, name));
        }
        else{
            if *exp > max {
            let max = exp;
            let dlt = choice.pop();
            choice.push((*exp, name));
            }
        }
    }
    for (exp, name) in choice{
        println!("The applicant with the highest years of experience and as such the best fit for the job is {name}, with experience of {exp} years.");
    }
        

}
