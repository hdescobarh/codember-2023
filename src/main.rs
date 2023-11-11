mod challenge01;
mod challenge02;
use std::fs;

fn main() {
    // Challenge 01 — Encrypted message
    println!("Solving Challenge 01...");
    let message = fs::read_to_string("./data/message_01.txt").expect("Failed to read the file");
    println!("{}\n", challenge01::decoder(&message));

    // Challenge 02 — Mini compiler
    println!("Solving Challenge 02...");
    let message = fs::read_to_string("./data/message_02.txt").expect("Failed to read the file");
    let mut compiler = challenge02::Compiler::new();
    println!("{}", compiler.build(&message));
}
