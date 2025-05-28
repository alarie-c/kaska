use std::fmt::Display;
use super::span::{ formatted_content, line_number, Span };

// ----------------------------------------------------------------- \\
// MACROS
// ----------------------------------------------------------------- \\

#[macro_export]
macro_rules! throw {
    ($kind:ident, $span:expr, $msg:literal) => {
        Error::new(ErrorKind::$kind, $span, $msg.to_string(), true)
    };
    ($kind:ident, $span:expr, $msg:expr) => {
        Error::new(ErrorKind::$kind, $span, $msg, true)
    };
}

// ----------------------------------------------------------------- \\
// ERROR WRITER
// ----------------------------------------------------------------- \\

pub type ErrorBuffer = Vec<Error>;

pub trait ErrorWriter {
    fn error(&mut self, error: Error);
    fn dump_errors(&mut self) -> ErrorBuffer;
}

// ----------------------------------------------------------------- \\
// ERROR STRUCT
// ----------------------------------------------------------------- \\

#[derive(Debug)]
pub struct Error {
    /// Refers to the kind of error and may or may not have
    /// some extra data that will be used for reporting
    kind: ErrorKind,

    /// Refers to the offending part of the code, this is the part
    /// that will be underlined with carets when reported
    span: Span,

    /// The help message to be printed underneath the offending code
    /// when this error is reported
    msg: String,

    /// Whether or not this error will abort compilation
    abort: bool,
}

impl Error {
    /// Quick way to create a new error, default constructor
    pub fn new(kind: ErrorKind, span: Span, msg: String, abort: bool) -> Error {
        Error {
            kind,
            span,
            msg,
            abort,
        }
    }
}

// ----------------------------------------------------------------- \\
// ERROR KINDS
// ----------------------------------------------------------------- \\

#[derive(Debug)]
pub enum ErrorKind {
    // Errors
    IllegalCharacter,
    SyntaxError,
    ParseError,
    TypeMismatch,
    AssignToConstant,
    UnknownIdentifier,

    // Warnings
    UnusedVariable,
    RedundantCode,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IllegalCharacter => write!(f, "found illegal character"),
            Self::SyntaxError => write!(f, "syntax error"),
            Self::ParseError => write!(f, "parse error"),
            Self::TypeMismatch => write!(f, "type mistmatch"),
            Self::AssignToConstant => write!(f, "tried to assign to a constant"),
            Self::UnknownIdentifier => write!(f, "unknown identifier"),
            Self::UnusedVariable => write!(f, "unused variable"),
            Self::RedundantCode => write!(f, "redundant code"),
        }
    }
}
