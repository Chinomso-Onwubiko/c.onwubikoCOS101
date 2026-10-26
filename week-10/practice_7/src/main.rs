struct Employee {
    name: String,
    company: String,
    age: u32
}

fn main() {
    let emp1 = Employee{
        name: String::from("Ebibiong Jessica"),
        company: String::from("Enrst & Young"),
        age:25
    };

    println!("Name = {}", emp1.name);
    println!("Company = {}", emp1.company);
    println!("Age = {}", emp1.age);
    
}
