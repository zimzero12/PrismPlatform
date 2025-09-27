// in src/main.rs

// We only need these top-level imports for our application logic.
use std::env;
use std::fs;
use std::io::{self, Write};

// The 'use' line is the most important change.
// It says: "From our 'prism_platform' library, import these specific tools."
// This means we no longer need 'mod' declarations here.
use prism_platform::{
    evaluator::Evaluator,
    lexer::Lexer,
    parser::Parser,
    error::TraceResult,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        run_from_file(&args[1]);
    } else {
        run_repl();
    }
}

fn run_from_file(filename: &str) {
    let source_code = match fs::read_to_string(filename) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Application Error: Could not read file '{}': {}", filename, e);
            return;
        }
    };
    let mut evaluator = Evaluator::new();
    if let Err(e) = execute(&source_code, &mut evaluator) {
        eprintln!("{}", e);
    }
}

fn run_repl() {
    println!("Trace Language Interpreter v0.6 (Correct Structure)");
    println!("Type 'exit' to close the interpreter.");
    let mut evaluator = Evaluator::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            continue;
        }
        if trimmed_input == "exit" {
            println!("Goodbye!");
            break;
        }
        if let Err(e) = execute(trimmed_input, &mut evaluator) {
            eprintln!("{}", e);
        }
    }
}

fn execute(source: &str, evaluator: &mut Evaluator) -> TraceResult<()> {
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    evaluator.eval_program(&program)?;
    Ok(())
}