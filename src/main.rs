// in src/main.rs

// We need the standard library's io module to read from the command line
use std::io::{self, Write};

mod ast;
mod evaluator;
mod lexer;
mod parser;

// Bring our tools into scope
use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;
use crate::Token::{EndOfFile, Identifier, Say, Text, Number, Equals, Plus, LessThan, Create, Loop, If};

// The Token enum remains unchanged, but we still need it here.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Say, Create, Loop, If,
    Identifier(String), Text(String), Number(f64),
    Equals, Plus, LessThan,
    EndOfFile,
}


fn main() {
    println!("Trace Language Interpreter v0.2");
    println!("Type 'exit' to close the interpreter.");

    // 1. Create a single, long-lived evaluator.
    // We make it mutable so its environment (memory) persists between commands.
    let mut evaluator = Evaluator::new();

    // 2. Start an infinite loop to read and execute commands.
    loop {
        // 3. Print a prompt to the user.
        print!("> ");
        // We must "flush" stdout to make sure the ">" appears before we wait for input.
        io::stdout().flush().expect("Failed to flush stdout");

        // 4. Read a line of input from the user.
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // 5. Check for the exit command.
        if input.trim() == "exit" {
            println!("Goodbye!");
            break; // Exit the loop
        }

        // 6. Feed the user's input into our language pipeline!
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        // 7. Evaluate the parsed program. The evaluator will print any `say` output.
        evaluator.eval_program(&program);
    }
}