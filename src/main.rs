use std::fs;
use common::errors::ErrorBuffer;
use lexer::{ lexer::Lexer, token::Token };
use parser::{ ast::Stmt, parser::Parser };

mod common;
mod lexer;
mod parser;
mod analysis;
mod tests;

const PATH: &'static str = "main.kas";

fn lex(source_code: &String) -> (Vec<Token>, ErrorBuffer) {
    let mut lexer = Lexer::new(&source_code);
    return lexer.lex();
}

pub fn parse(source: Vec<Token>) -> (Vec<Stmt>, ErrorBuffer) {
    let mut parser = Parser::new(source);
    return parser.parse();
}

fn main() {
    let source_code = fs::read_to_string(&PATH).expect("There was an error reading the file!");

    // tokenize and debug
    let (tokens, lex_errs) = lex(&source_code);
    println!("Tokens:\n{:#?}", tokens);

    // parse and debug
    let (ast, parse_errs) = parse(tokens);
    println!("AST:\n{:#?}", ast);

    // print errors
    println!("Errors:");
    println!("{:#?}", lex_errs);
    println!("{:#?}", parse_errs);
}
