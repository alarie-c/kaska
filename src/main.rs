use std::fs;
use common::errors::{ check_errs_for_abort, ErrorBuffer };
use compiler::{ lexer::{ Lexer, Token }, parser };

mod common;
mod compiler;

const PATH: &'static str = "main.ks";

fn lex(source_code: &String) -> (Vec<Token>, ErrorBuffer) {
    let mut lexer = Lexer::new(&source_code);
    return lexer.lex();
}

fn main() {
    let source_code = fs::read_to_string(&PATH).expect("There was an error reading the file!");

    // tokenize and debug
    let (tokens, lex_errs) = lex(&source_code);
    println!("[[ TOKENS ]{:#?}", tokens);

    // parse and debug
    let (ast, parse_errs) = parser::parse(tokens);
    println!("[[ AST ]{:#?}", ast);

    // print errors
    print!("[[ ERRORS ]");
    print!("{:#?}", lex_errs);
    print!("{:#?}\n", parse_errs);

    // determine testing error code from errors
    let code: i32 = match check_errs_for_abort(&lex_errs) || check_errs_for_abort(&parse_errs) {
        true => -1,
        false => 0,
    };

    std::process::exit(code);
}
