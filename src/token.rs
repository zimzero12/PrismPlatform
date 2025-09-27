// in src/token.rs
#![allow(dead_code)]

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