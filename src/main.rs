#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut token_type: String;
            let mut lexeme: String;
            let mut literal: String;
            let mut had_error: bool = false;

            // TODO: Uncomment the code below to pass the first stage
            if !file_contents.is_empty() {
               // panic!("Scanner not implemented");
               for chars in file_contents.chars() {
                   match chars {
                       '(' => println!("LEFT_PAREN ( null"),
                       ')' => println!("RIGHT_PAREN ) null"),
                       '{' => println!("LEFT_BRACE {{ null"),
                       '}' => println!("RIGHT_BRACE }} null"),
                       ',' => println!("COMMA , null"),
                       '.' => println!("DOT . null"),
                       '-' => println!("MINUS - null"),
                       '+' => println!("PLUS + null"),
                       ';' => println!("SEMICOLON ; null"),
                       '/' => println!("SLASH / null"),
                       '*' => println!("STAR * null"),
                       _ => {
                           eprintln!("[line 1] Error: Unexpected character: {}", chars);
                           had_error = true;
                       }
                   }
               }
               println!("EOF  null")
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
            if had_error {
                process::exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn error(line: i32, message: String) {
    report(line, "", message);
}

fn report(line: i32, location: &str, message: String) {
    println!("[line {}{}{}{}{}", line.to_string(), "] Error", location, ": ", message);
    // hadError = true;
}
