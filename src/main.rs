// Dependencies
use std::io::stdin;
use hex::encode;
use hex::decode;



// Colors
const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const BLUE: &str = "\x1b[94m";
const MAGENTA: &str = "\x1b[95m";



// Encode
fn p2h() {
    println!("\n{}Plain Text String: ", YELLOW);

    let mut input = String::new(); // Creates new string

    // Read input
    stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let input = input.trim();

    println!("\n{}{}", GREEN, encode(input)); // Encode
}



// Decode
fn h2p() {
    println!("\n{}Hex String(Without Padding): ", BLUE);

    let mut input = String::new(); // Creates new string

    // Read input
    stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let hex = input.trim();

    println!("\n{}{:?}", GREEN, decode(hex)); // Decode
}



// Prompt
fn prompt() {
    println!("{}1. From Plain Text to hexidecimals\n", YELLOW);
    println!("{}2. From Hexidecimals to Plain text\n\n", BLUE);
    println!("{}hexl > ", MAGENTA);

    let mut input = String::new(); // Creates new string

    // Read input
    stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    // Interpret input
    match &input.trim()[..] {
        "1" => p2h(),
        "2" => h2p(),
        _ => println!("{}Invalid Options!", RED),
    };
}



// Main
#[allow(unused_variables)]
fn main() {

    prompt();
}