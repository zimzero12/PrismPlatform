// in src/evaluator.rs

use crate::ast::{Expression, Program, Statement};
use std::collections::HashMap; // Import HashMap!

// The environment holds our variables. It's a map from a string (name) to a value (f64).
type Environment = HashMap<String, f64>;

pub struct Evaluator {
    environment: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            environment: HashMap::new(),
        }
    }

    pub fn eval_program(&mut self, program: &Program) {
        for statement in &program.statements {
            self.eval_statement(statement);
        }
    }

    fn eval_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::SayStatement { value } => {
                println!("{}", value);
            }
            Statement::CreateStatement { name, value } => {
                // When we see a create statement...
                let value_to_store = self.eval_expression(value);
                // ...we insert the variable's name and its evaluated value into our environment!
                self.environment.insert(name.clone(), value_to_store);
            }
        }
    }

    // NEW FUNCTION to evaluate expressions and get their concrete value
    fn eval_expression(&self, expression: &Expression) -> f64 {
        match expression {
            Expression::NumberLiteral(value) => *value,
        }
    }
}