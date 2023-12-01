mod challenge01;
mod challenge02;
mod challenge03;
mod challenge04;
mod challenge05;
use std::fs;

const FILE_ERR: &str = "Failed to read the file";
fn main() {
    // Challenge 01 — Encrypted message
    println!("Solving Challenge 01...");
    let message = fs::read_to_string("./data/message_01.txt").expect(FILE_ERR);
    println!("{}\n", challenge01::decoder(&message));

    // Challenge 02 — Mini compiler
    println!("Solving Challenge 02...");
    let message = fs::read_to_string("./data/message_02.txt").expect(FILE_ERR);
    let mut compiler = challenge02::Compiler::new();
    println!("{}", compiler.build(&message));

    // Challenge 03 - Spy encryption
    println!("Solving Challenge 03...");
    let policies = fs::read_to_string("./data/encryption_policies.txt").expect(FILE_ERR);
    let validation = challenge03::validate_policies(&policies);
    println!("The 42nd invalid key is: {}", validation[41]);

    // Challenge 04 - Hacker infiltration
    println!("Solving Challenge 04...");
    let files_quarantine = fs::read_to_string("./data/files_quarantine.txt").expect(FILE_ERR);
    let validation = challenge04::validate_files(&files_quarantine);
    println!("The 33rd real file checksum is: {}", validation[32]);

    // Challenge 05 - The Final Problem
    println!("Solving Challenge 05...");
    let csv_content = fs::read_to_string("./data/database_attacked.txt").expect(FILE_ERR);
    let message = challenge05::discover_message(&csv_content);
    println!("The message is: {}", message);
}
