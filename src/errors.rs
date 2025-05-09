use std::fmt::Display;
use crate::span::{formatted_content, line_number, Span};

pub type ErrorBuffer = Vec<Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    span: Span,
    underline: Span,
    msg: String,
    abort: bool,
}

impl Error {
    pub fn new(kind: ErrorKind, span: Span, underline: Span, msg: &str, abort: bool) -> Error {
        Error {
            kind,
            span,
            underline,
            msg: msg.to_string(),
            abort,
        }
    }

    pub fn report(&self, source: &String) {
        let line_number = line_number(&self.span, source);
        let content = formatted_content(&self.span, &self.underline, source);

        println!("[Error] line {}: {}", line_number, self.kind);
        if content.is_some() {
            println!("{}", content.unwrap());
        }
        println!("{}", self.msg);
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    IllegalCharacter,
    SyntaxError,
    ParseError,
    TypeMismatch,
    AssignToConstant,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::IllegalCharacter => write!(f, "found illegal character"),
            ErrorKind::SyntaxError => write!(f, "syntax error"),
            ErrorKind::ParseError => write!(f, "parse error"),
            ErrorKind::TypeMismatch => write!(f, "type mistmatch"),
            ErrorKind::AssignToConstant => write!(f, "tried to assign to a constant"),
        }
    }
}