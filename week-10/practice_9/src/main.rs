//Define dimensions of a rectangle
struct Rectangle {
    width: u32,
    height: u32
}


//logic to calculate area of a rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        //use the .operator to fetch the value of a field via the self keyword.
        self.width * self.height
    }
}

fn main() {
    //instantiate the structure
    let small = Rectangle {
        width: 10,
        height: 20
    };

    let area = small.area();
    println!("Width is {} \nHeight is {} \nArea of the rectangle is {}", small.width, small.height, area);
}
