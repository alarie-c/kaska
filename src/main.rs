use std::fs;
use common::errors::{ check_errs_for_abort, ErrorBuffer };
use compiler::lexer::{ Lexer, Token };

mod common;
mod compiler;

const PATH: &'static str = "main.kas";

fn lex(source_code: &String) -> (Vec<Token>, ErrorBuffer) {
    let mut lexer = Lexer::new(&source_code);
    return lexer.lex();
}

// pub fn parse(source: Vec<Token>) -> (AST, ErrorBuffer) {
//     let mut parser = Parser::new(source);
//     let ast = parser.parse();

//     let mut errors: ErrorBuffer = vec![];
//     let _ = std::mem::replace(&mut errors, parser.errors);
//     return (ast, errors);
// }

fn main() {
    let source_code = fs::read_to_string(&PATH).expect("There was an error reading the file!");

    // tokenize and debug
    let (tokens, _) = lex(&source_code);
    println!("[[ TOKENS ]{:#?}", tokens);

    // // parse and debug
    // let (ast, parse_errs) = parse(tokens);
    // println!("[[ AST ]{:#?}", ast);

    // // print errors
    // print!("[[ ERRORS ]");
    // print!("{:#?}", lex_errs);
    // print!("{:#?}\n", parse_errs);

    // // determine testing error code from errors
    // let code: i32 = match check_errs_for_abort(&lex_errs) || check_errs_for_abort(&parse_errs) {
    //     true => -1,
    //     false => 0,
    // };

    // std::process::exit(code);
}
