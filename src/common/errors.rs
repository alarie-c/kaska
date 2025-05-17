use std::fmt::Display;
use super::span::Span;

pub type ErrorBuffer = Vec<Error>;

/// Takes an error buffer and returns true of one or more of the errors
/// will abort compilation
pub fn check_errs_for_abort(buffer: &ErrorBuffer) -> bool {
    return buffer.iter().any(|err| err.abort == true);
}

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
    pub fn new(kind: ErrorKind, span: Span, msg: &str, abort: bool) -> Error {
        Error {
            kind,
            span,
            msg: msg.to_string(),
            abort,
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    IllegalCharacter,
    SyntaxError,
    ParseError,
    TypeMismatch,
    AssignToConstant,
    UnknownIdentifier,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::IllegalCharacter => write!(f, "found illegal character"),
            ErrorKind::SyntaxError => write!(f, "syntax error"),
            ErrorKind::ParseError => write!(f, "parse error"),
            ErrorKind::TypeMismatch => write!(f, "type mistmatch"),
            ErrorKind::AssignToConstant => write!(f, "tried to assign to a constant"),
            ErrorKind::UnknownIdentifier => write!(f, "unknown identifier"),
        }
    }
}
