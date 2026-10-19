use::std::fs;
use::std::io::Write;
use::std::io::Read;

fn main() {
    //Creates a file named 'data.txt'
    let mut file = fs::File::create("data.txt").expect("Couldn't create.");
    file.write_all("Hi there, this is just a test.".as_bytes()).expect("Couldn't write the first line");
    file.write_all("This will continue on the same line as the previously appended line.".as_bytes()).expect("Couldn't write the second line.");
    println!("File was created, here are its contents: ");

    //To view its contents:
    let  mut viewer = fs::File::open("data.txt").expect("The file could not be opened.");
    let mut content = String::from("");
    viewer.read_to_string(&mut content).unwrap();
    println!("{}", content);

    //deletes the file 'data.txt'
    fs::remove_file("data.txt").expect("Could not remove file.");
    println!("File is removed");  
}
