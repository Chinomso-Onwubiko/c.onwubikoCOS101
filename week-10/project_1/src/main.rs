struct Laptops{
    brand: String,
    quantity: u32,
    price: u32
}

impl Laptops{
    fn cost(&self, num_bought:u32) -> u32 {
       let cost = self.price * num_bought;
       cost
    }
}

fn main() {
    println!("Welcome to Ogbeifuna Enterprise, your order is being processed...");

    let set1 = Laptops{
        brand: "HP".to_string(),
        quantity: 10,
        price: 650_000
    };

    let set2 = Laptops{
        brand: String::from("IBM"),
        quantity: 6,
        price: 755_000
    };

    let set3 = Laptops{
        brand: "Toshiba".to_string(),
        quantity: 10,
        price: 550_000
    };

    let set4 = Laptops {
        brand: "Dell".to_owned(),
        quantity: 4,
        price: 850_000
    };

    let total = set1.cost(3) + set2.cost(3) + set3.cost(3) + set4.cost(3);
    println!("The cost of the 3 {} laptops is {}", set1.brand, set1.cost(3));
    println!("The cost of the 3 {} laptops is {}", set2.brand, set2.cost(3));
    println!("The cost of the 3 {} laptops is {}", set3.brand, set3.cost(3));
    println!("The cost of the 3 {} laptops is {}\n", set4.brand, set4.cost(4));


    println!("The total cost of all the laptops you bought today is N{}", total);
    println!("Thank you for doing business with the Ogbeifuna Enterprise, we look forward to serving you again.🤝🤝🤝")
}
