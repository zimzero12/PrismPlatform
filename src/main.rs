mod ast;
mod evaluator; // We've added our new module
mod lexer;
mod parser;

// Bring our tools into scope
use evaluator::Evaluator; // Import the Evaluator
use lexer::Lexer;
use parser::Parser;
use crate::Token::{EndOfFile, Identifier, Say, Text, Number, Equals, Plus, LessThan, Create, Loop, If};


#[derive(Debug, PartialEq, Clone)]
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
    println!("--- Running Trace Language ---");

    // Let's give it a real program to run!
    let source_code = String::from("say \"Trace is now a real, functional language!\"");

    // 1. Lexer: Turn the code into tokens (words).
    let lexer = Lexer::new(source_code);

    // 2. Parser: Turn the tokens into an AST (sentences).
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    // 3. Evaluator: Take the AST and execute it!
    let evaluator = Evaluator::new();
    evaluator.eval_program(&program);
}