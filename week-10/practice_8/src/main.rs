//A strunct travels into a function the way typles did in week 8:
//as one argument.

//Declare a struct:
struct Employee {
    ceo: String,
    company: String,
    age: u32
}

fn main() {
    //initialize or instantiate a structure:
    let emp1 = Employee {
        company: "Microsoft Corporation".to_string(),
        ceo: "Satya Nadella".to_string(),
        age: 56
    };

    let emp2 = Employee {
        ceo: "Sundai Pichai".to_owned(),
        company: String::from("Google Inc."),
        age:51
    };
    //pass emp1 and emp2 to display()
    display(emp1);
    display(emp2);
}

//fetch values of specific structure fields using the operator and print
//it to the console.

fn display(emp: Employee) {
    println!("Name is {}\ncompany is {} \nage is {}", emp.ceo, emp.company, emp.age);
    println!("\n");

}