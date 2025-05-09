use std::{ env, fs };

use ast::parser::Parser;
use lexer::Lexer;
use token::Token;

mod span;
mod token;
mod lexer;
mod ast;
mod analysis;

/// Opens the file path and returns the contents as a `String`.
/// Panics on error.
fn open_file(file_path: &String) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("There was an error reading the file!");
    })
}

fn main() {
    // let args: Vec<String> = env::args().collect();

    // get the file from the arguments
    // let file_path = args.get(1).unwrap_or_else(|| {
    //     panic!("No file path specified!");
    // });

    let file_path = String::from("main.ks");

    let source_code = open_file(&file_path);
    println!("Source:\n{}", source_code);

    let mut lexer = Lexer::new(&source_code);
    lexer.lex();
    lexer.dump();

    // move tokens out of lexer
    let mut tokens: Vec<Token> = vec![];
    _ = std::mem::replace(&mut tokens, lexer.tokens);

    let mut parser = Parser::new(tokens);
    let exprvec = parser.parse();

    println!("{:#?}", exprvec);
}
