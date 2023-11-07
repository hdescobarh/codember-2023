mod challenge01;
use std::fs;

fn main() {
    // Challenge_01
    let message = fs::read_to_string("./data/message_01.txt").expect("Failed to read the file");
    println!("{}", challenge01::decoder(&message))
}
