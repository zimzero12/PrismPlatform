// This enum represents all the "words" our language understands.
// We'll add more to this list as our language grows.
#[derive(Debug)] // This lets us print the tokens for testing
enum Token {
    // Keywords
    Say,          // for the "say" command
    Create,       // for the "create" command
    Loop,         // for the "loop" command
    If,           // for the "if" command

    // Identifiers and Literals
    Identifier(String), // for variable names like "player_guess"
    Text(String),       // for strings in double quotes like "Hello, World!"
    Number(f64),        // for numbers like 100 or 3.14

    // Operators and Symbols
    Equals,       // for the = sign
    Plus,         // for the + sign
    LessThan,     // for the < sign

    // A special token to mark the end of the code
    EndOfFile,
}

fn main() {
    // For now, our main function is empty.
    // We'll add code here later to actually *use* our tokens.
    println!("Trace language token definitions are ready!");
    println!("Next step will be to build the lexer to create these tokens from code.");
}