 use std::fs::File;
 use std::io::Write;
 use std::io::Read;


 fn main() {
    let mut comms = File::open("commissioners.csv").unwrap(); //This opens the files and gives us access to read them.
    let mut mins = File::open("ministry.csv").unwrap();
    let mut zones = File::open("zone.csv").unwrap();

    let mut comm = String::new(); //This creats a new empty string object.
    let mut min = String::new();
    let mut zone = String::new();

    let mut merged = File::create("Merged_ministers.csv").expect("Could not create the merged csv file."); //This creates a new document with the given name
    //and file extension.

    comms.read_to_string(&mut comm).unwrap(); //This converts the binary data in the csv files and converts them strings which are stored in the variables
    //passed as parameters.
    mins.read_to_string(&mut min).expect("Your list of ministers was unable to be stored.");
    zones.read_to_string(&mut zone).unwrap();

    let commissioners: Vec<&str> = comm.trim().lines().collect();//We use the .lines() method because it handles the splitting of the text by taking into
    //account both "\r" and "\n".
    let ministry: Vec<&str> = min.trim().lines().collect();
    let zonal: Vec<&str> = zone.trim().lines().collect();
    println!("{:?}\n {:?}\n {:?}", zonal, ministry, commissioners);
    merged.write_all("Commissioners, Ministers, Zone, \n".as_bytes()).unwrap(); //This writes the headers of the columns to the merged csv file.

    let mut count = 0;

    while count < commissioners.len(){ //This appends the appropriate data to their rightful indexes under their respective columns in the csv file.
        merged.write_all(format!("{},{},{},\n",commissioners[count], ministry[count], zonal[count] ).as_bytes()).unwrap();
        count += 1;
    }
}
