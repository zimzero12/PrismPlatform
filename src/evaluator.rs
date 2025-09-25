// We need to know about the structures we're going to evaluate.
use crate::ast::{Program, Statement};

// The Evaluator struct. For now, it doesn't need to hold any data.
pub struct Evaluator;

impl Evaluator {
    // A function to create a new Evaluator.
    pub fn new() -> Self {
        Evaluator
    }

    // This is the main entry point. It takes a whole program (the AST) and evaluates it.
    pub fn eval_program(&self, program: &Program) {
        // We simply loop through all the statements in the program and evaluate them one by one.
        for statement in &program.statements {
            self.eval_statement(statement);
        }
    }

    // This function handles individual statements.
    fn eval_statement(&self, statement: &Statement) {
        // We use a match statement to determine what kind of statement we're looking at.
        match statement {
            // If we find a SayStatement...
            Statement::SayStatement { value } => {
                // ...we simply print its value to the console! This is the magic.
                println!("{}", value);
            }
        }
    }
}