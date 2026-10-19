use std::fs::File; 
use std::io::Read; 
use std::io::Write;

fn main() {
    let columns: [&str; 3] = ["Lager, ", "Stout, ", "Non-Alcoholic"];
    let lager: Vec<&str> = vec!["33 Export, ", "Desperados, ", "Goldberg, ", "Gulder, ", "Heineken, ", "Star, "];
    let stout: Vec<&str> = vec!["Legend, ", "Turbo King, ", "Williams, ", " ,", " ,", " ," ];
    let non_a: Vec<&str> = vec![ "Maltina, ", "Amstel Malta, ", "Malta Gold, ", "Fayrouz ,", " ,", " ,"];

    let mut csv_file = File::create("Drink_categories.csv").expect("Couldn't create the csv file.");
    for col in columns{ //This inserts the drink categories into the csv file, before the various drinks are appended accordingly.
        csv_file.write_all(col.as_bytes()).expect("Couldn't write the column to the csv file.");
    }

    for i in 0..6{ //The first line appends a new line for every round of the loop, this helps ensure the drinks are place under the right category after each loop.
        //The remaining lines of code under the loop add the respective drinks under their appropriate categories, one after the other.
        csv_file.write_all("\n".as_bytes()).expect("Couldn't append a new line.");
        csv_file.write_all(lager[i].as_bytes()).expect("Couldn't append for lager.");
        csv_file.write_all(stout[i].as_bytes()).expect("Couldn't append for stout.");
        csv_file.write_all(non_a[i].as_bytes()).expect("Couldn't append for non Non-Alcoholic");
    }

    let mut file = File::open("Drink_categories.csv").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("Here is a rough view of the categories of drinks present alongside the drinks in stock under each category.");
    println!("{}", contents); //This is just to confirm if my visual image of what I wanted the csv file to look like was achieved,
    //the actual csv file is properly organised when viewed with a spreadsheet app which displays the comma separated values in columns and rows.
}
