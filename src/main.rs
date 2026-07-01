#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process;

pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
}

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

            let file_content = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut token_type: String;
            let mut lexeme: String;
            let mut literal: String;
            let mut had_error: bool = false;

            // TODO: Uncomment the code below to pass the first stage
            if !file_content.is_empty() {
               // panic!("Scanner not implemented");
               let mut chars = file_content.chars().peekable();
               while let Some(character) = chars.next() {
                   match character {
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
                       // Assignemnt and equality operators
                       '=' => {
                           if chars.peek() == Some(&'=') { // Use & since peek() gives a reference
                               chars.next();
                               println!("EQUAL_EQUAL == null");
                           } else {
                               println!("EQUAL = null");
                           }
                       },
                       // Inequality and negation operators
                       '!' => {
                           if chars.peek() == Some(&'=') {
                               chars.next();
                               println!("BANG_EQUAL != null");
                           } else {
                               println!("BANG ! null");
                           }
                       },
                       // Relational operators
                       '>' => {
                           if chars.peek() == Some(&'=') {
                               chars.next();
                               println!("GREATER_EQUAL >= null");
                           } else {
                               println!("GREATER > null");
                           }
                       },
                       '<' => {
                           if chars.peek() == Some(&'=') {
                               chars.next();
                               println!("LESS_EQUAL <= null");
                           } else {
                               println!("LESS < null");
                           }
                       },

                       _ => {
                           eprintln!("[line 1] Error: Unexpected character: {}", character);
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
