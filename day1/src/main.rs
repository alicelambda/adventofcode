use std::env;
use std::fs;


fn main() {

    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");

    let my_int = contents.parse::<i32>().unwrap();
    println!("{}",contents);
}
