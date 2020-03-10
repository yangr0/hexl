// Dependencies
use std::io::stdin;

// Main
#[allow(unused_variables)]
fn main() {
    // Colors
    let red = "\x1b[91m";
    let green = "\x1b[92m";
    let yellow = "\x1b[93m";
    let blue = "\x1b[94m";
    let magenta = "\x1b[95m";
    let cyan = "\x1b[96m";

    println!("{}1. From Plain Text to hexidecimals\n", green);
    println!("{}2. From Hexidecimals to Plain text\n\n", blue);
    println!("{}hexl > ", red);

    let mut input = String::new(); // Creates new string

    stdin().read_line(&mut input) // Reads the string
        .ok()
        .expect("Failed to read line"); // Error handling

    // Interpret input
    match &input.trim()[..] {
        "1" => 
    };

}