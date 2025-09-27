use crate::ast::{Expression, Program, Statement};
use std::collections::HashMap;

// An enum to represent the different types of values in Trace.
#[derive(Debug, Clone)]
pub enum TraceValue {
    Number(f64),
    Text(String),
}

// The environment now stores our new TraceValue type.
type Environment = HashMap<String, TraceValue>;

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
            Statement::CreateStatement { name, value } => {
                let value_to_store = self.eval_expression(value);
                self.environment.insert(name.clone(), value_to_store);
            }
            Statement::SayStatement { value } => {
                // First, evaluate the expression to get a concrete value.
                let value_to_print = self.eval_expression(value);
                // Now, print that value based on its type.
                match value_to_print {
                    TraceValue::Number(n) => println!("{}", n),
                    TraceValue::Text(t) => println!("{}", t),
                }
            }
        }
    }

    fn eval_expression(&self, expression: &Expression) -> TraceValue {
        match expression {
            Expression::NumberLiteral(value) => TraceValue::Number(*value),
            Expression::TextLiteral(value) => TraceValue::Text(value.clone()),
            Expression::Identifier(name) => {
                match self.environment.get(name) {
                    Some(value) => value.clone(), // Return the found value
                    None => {
                        // If the variable isn't found, print an error and return a default value.
                        println!("Error: Variable '{}' not found.", name);
                        TraceValue::Text(String::from("undefined"))
                    }
                }
            }
        }
    }
}