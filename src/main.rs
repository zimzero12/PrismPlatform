mod ast;
mod evaluator; // We've added our new module
mod lexer;
mod parser;

// Bring our tools into scope
use evaluator::Evaluator; // Import the Evaluator
use lexer::Lexer;
use parser::Parser;
use crate::Token::{EndOfFile, Identifier, Say, Text, Number, Equals, Plus, LessThan, Create, Loop, If};


// in src/main.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Say,
    Create, // <-- ADDED
    Loop,
    If,

    // Identifiers and Literals
    Identifier(String),
    Text(String),
    Number(f64), // <-- ADDED (was already here, but make sure!)

    // Operators and Symbols
    Equals, // <-- ADDED
    Plus,
    LessThan,

    // A special token to mark the end of the code
    EndOfFile,
}

// in src/main.rs (at the bottom)

fn main() {
    println!("--- Running Trace Language v0.2 ---");

    // Our new program using variables!
    let source_code = String::from("create score = 100");

    let lexer = Lexer::new(source_code);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    // We need a mutable evaluator now since it stores variables
    let mut evaluator = Evaluator::new();
    evaluator.eval_program(&program);

    // We can't see the result yet, but if this runs without crashing, it worked!
    println!("Program finished. Variable 'score' should now be stored.");
}