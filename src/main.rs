use std::{ env, fs };
use analysis::resolver::Resolver;
use ast::{parser::Parser, stmt::Stmt};
use compile::frontend::Compiler;
use errors::ErrorBuffer;
use lexer::Lexer;
use token::Token;

mod span;
mod token;
mod lexer;
mod ast;
mod analysis;
mod errors;
mod compile;

const PATH: &'static str = "main.ks";

/// Opens the file path and returns the contents as a `String`.
/// Panics on error.
fn open_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("There was an error reading the file!");
    })
}

fn main() {
    let source_code = open_file(&PATH);
    println!("Source:\n{}", source_code);

    let mut lexer = Lexer::new(&source_code);
    lexer.lex();
    lexer.dump();

    // move tokens out of lexer
    let mut tokens: Vec<Token> = vec![];
    _ = std::mem::replace(&mut tokens, lexer.tokens);

    // parse and print errors
    let mut parser = Parser::new(tokens);
    let (ast, parser_errors) = parser.parse();
    println!("AST:\n{:#?}", ast);
    println!("PARSER ERRORS:\n{:#?}", parser_errors);

    // name resolution and semantic analysis
    let mut resolver = Resolver::new(&ast);
    let resolver_errors: ErrorBuffer = resolver.resolve();
    println!("SEMANTIC ERRORS:\n{:#?}", resolver_errors);

    // fire up the compiler and write some bytecode YEAAAAH
    let mut compiler = Compiler::new(&ast);
    let _ = compiler.compile(String::from("main"));
}
