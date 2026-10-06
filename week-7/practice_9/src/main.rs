fn main() {
    let arr: [i32; 4] = [10, 20, 30, 40];
    let iter_arr = arr.iter();

    for numbs in iter_arr {
        println!("Value is {}", numbs);
    }
}
