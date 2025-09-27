// in src/main.rs --- FINAL VERSION

use std::env;
use std::fs;
use std::io::{self, Write};

mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // --- FILE MODE ---
        let filename = &args[1];
        run_from_file(filename);
    } else {
        // --- REPL MODE ---
        run_repl();
    }
}

fn run_from_file(filename: &str) {
    let source_code = match fs::read_to_string(filename) {
        Ok(code) => code,
        Err(_) => {
            eprintln!("Error: Could not read file '{}'", filename);
            return;
        }
    };

    let lexer = Lexer::new(source_code);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let mut evaluator = Evaluator::new();
    evaluator.eval_program(&program);
}

fn run_repl() {
    println!("Trace Language Interpreter v0.4");
    println!("Type 'exit' to close the interpreter.");

    let mut evaluator = Evaluator::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "exit" {
            println!("Goodbye!");
            break;
        }

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        evaluator.eval_program(&program);
    }
}