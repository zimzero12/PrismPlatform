// We now have three modules: ast, lexer, and parser
mod ast;
mod lexer;
mod parser;

// Bring our tools into scope
use lexer::Lexer;
use parser::Parser;
use crate::Token::{EndOfFile, Identifier, Say, Text, Number, Equals, Plus, LessThan, Create, Loop, If};


#[derive(Debug, PartialEq, Clone)] // We need Clone now for the parser
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
    println!("--- Running Trace Parser ---");

    // Our line of Trace code
    let source_code = String::from("say \"Hello from the Parser!\"");

    // 1. Create the Lexer
    let lexer = Lexer::new(source_code);

    // 2. Create the Parser
    let mut parser = Parser::new(lexer);

    // 3. Parse the program to build the AST
    let program = parser.parse_program();

    // 4. Print the result!
    println!("Successfully parsed program:");
    println!("{:#?}", program); // The {:#?} makes the printout pretty
}