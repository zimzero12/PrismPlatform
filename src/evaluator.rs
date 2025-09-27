// in src/evaluator.rs

use crate::ast::{Expression, Program, Statement};
use crate::error::{EvalError, TraceResult}; // <-- ADDED
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TraceValue {
    Number(f64),
    Text(String),
}

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

    // eval_program now returns a Result
    pub fn eval_program(&mut self, program: &Program) -> TraceResult<()> {
        for statement in &program.statements {
            self.eval_statement(statement)?; // Propagate errors with '?'
        }
        Ok(())
    }

    // eval_statement now returns a Result
    fn eval_statement(&mut self, statement: &Statement) -> TraceResult<()> {
        match statement {
            Statement::CreateStatement { name, value } => {
                let value_to_store = self.eval_expression(value)?;
                self.environment.insert(name.clone(), value_to_store);
            }
            Statement::SayStatement { value } => {
                let value_to_print = self.eval_expression(value)?;
                match value_to_print {
                    TraceValue::Number(n) => println!("{}", n),
                    TraceValue::Text(t) => println!("{}", t),
                }
            }
        }
        Ok(())
    }

    // eval_expression now returns a Result
    fn eval_expression(&self, expression: &Expression) -> TraceResult<TraceValue> {
        match expression {
            Expression::NumberLiteral(value) => Ok(TraceValue::Number(*value)),
            Expression::TextLiteral(value) => Ok(TraceValue::Text(value.clone())),
            Expression::Identifier(name) => {
                match self.environment.get(name) {
                    Some(value) => Ok(value.clone()),
                    // THIS IS THE CRITICAL FIX: We now return a proper, structured error.
                    None => Err(Box::new(EvalError::VariableNotFound(name.clone()))),
                }
            }
        }
    }
}