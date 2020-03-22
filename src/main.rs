// Created by inc0gnit0
// https://github.com/iinc0gnit0/hexl



// Dependencies
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::exit;
use std::process::Command;
use hex::encode;
use hex::FromHex;



// Colors
const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const BLUE: &str = "\x1b[94m";
const MAGENTA: &str = "\x1b[95m";
const RESET: &str = "\x1b[0m";



// Exit
fn exitcode() {
    println!("{}Exiting...{}", RED, RESET);
    exit(0); 
}



// Encode
fn p2h() {
    print!("\n{}Plain Text String: ", YELLOW);
    stdout().flush().unwrap();

    let mut input = String::new(); // Creates new string

    // Read input
    stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let input = input.trim(); // Removes extra spaces

    println!("\n{}{}{}", GREEN, encode(input), RESET); // Encode
}



// Decode
fn h2p() {
    print!("\n{}Hex String(Without Padding): ", BLUE);
    stdout().flush().unwrap();

    let mut input = String::new(); // Creates new string

    // Read input
    stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let input = input.trim(); // Removes extra spaces

    // Decoding
    match Vec::from_hex(input) {
    Ok(vec) => {
        for b in vec {
            println!("{}{}", b as char, RESET);
        }
    }
    Err(_e) => {
    }
}
}



// Update
fn update() {
    Command::new("git")
    .arg("pull")
    .status()
    .expect("\x1b[91mFailed to check for updates");
    println!("{}", RESET); // Reset color
    exit(0);
}



// Prompt
fn prompt() {
    println!("{}https://github.com/iinc0gnit0/hexl
    
    {}         Hexl v1.7

{}[1] From Plain Text to hexidecimal
{}[2] From Hexidecimals to Plain text
{}[3] Check for updates
{}[x] Exit\n",GREEN, RED, YELLOW, BLUE, GREEN, RED);

    print!("{}[hexl]-> ", MAGENTA);
    stdout().flush().unwrap();

    let mut input = String::new(); // Creates new string

    // Read input
    stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    // Interpret input
    match &input.trim()[..] {
        "1" => p2h(),
        "2" => h2p(),
        "3" => update(),
        "x" => exitcode(),
        _ => println!("{}Invalid Option!{}", RED, RESET),
    };
}



// Main
fn main() {

    prompt(); // Calls prompt()
}