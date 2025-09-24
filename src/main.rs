// Declare that we have a lexer module (this tells main.rs that lexer.rs exists)
mod lexer;

// Bring our Token enum and the new Lexer struct into scope
use lexer::Lexer;
use crate::Token::{EndOfFile, Identifier, Say, Text};

// This enum represents all the "words" our language understands.
#[derive(Debug, PartialEq)] // Added PartialEq for easier testing later
pub enum Token {
    // Keywords
    Say,
    Create,
    Loop,
    If,

    // Identifiers and Literals
    Identifier(String),
    Text(String),
    Number(f64),

    // Operators and Symbols
    Equals,
    Plus,
    LessThan,

    // A special token to mark the end of the code
    EndOfFile,
}

fn main() {
    println!("--- Running Trace Lexer ---");

    // This is our first line of Trace code!
    let source_code = String::from("say \"Hello, Trace! This is working!\"");

    // Create a new lexer with our source code.
    let mut lexer = Lexer::new(source_code);

    // Loop until we get the EndOfFile token.
    loop {
        let token = lexer.next_token();
        println!("{:?}", token); // Print the token we found

        if token == EndOfFile {
            break; // Stop the loop
        }
    }
}