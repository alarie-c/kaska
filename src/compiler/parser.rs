use crate::{common::errors::{Error, ErrorBuffer, ErrorKind}, throw};
use super::{ast::{Expr, ExprKind}, lexer::{Token, TokenKind}};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        return Parser {
            tokens,
            pos: 0,
        };
    }

    pub fn parse(&mut self) {
        let mut ebuffer: ErrorBuffer = vec![];
        
    }
}

impl Parser {
    /// Returns whatever is at the current position of the parser.
    fn current(&self) -> &Token {
        return self.tokens.get(self.pos)
            .unwrap_or(&self.tokens[self.tokens.len()]);
    }

    /// Returns the next thing past the current position of the parser without changing the state of the parser.
    fn peek(&self) -> &Token {
        return self.tokens.get(self.pos + 1)
            .unwrap_or(&self.tokens[self.tokens.len()]);
    }

    /// Moves the position of the parser forward but will never exceed the EOF index.
    fn consume(&mut self) {
        self.pos = (self.pos + 1).clamp(0, self.tokens.len());
    }

    /// Expects the next token to be of the prescribed type and consume if it is. If it isn't, it will not consume but it will return an error.
    fn assert(&mut self, kind: TokenKind, msg: &str) -> Result<(), Error> {
        if self.peek().kind == kind {
            self.consume();
            return Ok(());
        }
        let span = self.current().span.clone();
        return Err(throw!(SyntaxError, span, msg.to_string()));
    }
}

impl Parser {
    fn expr_literal(&mut self) -> Result<Expr, Error> {
        let tk = self.current();

        match &tk.kind {
            TokenKind::Integer => {
                let value = match tk.lexeme.parse::<i32>() {
                    Ok(v) => v,
                    Err(_) => return Err(throw!(ParseError, tk.span.clone(), "error parsing this integer literal")),
                };
                return Ok(Expr::new(ExprKind::Integer { value }, tk.span.clone()));
            }
            TokenKind::Float => {
                let value = match tk.lexeme.parse::<f32>() {
                    Ok(v) => v,
                    Err(_) => return Err(throw!(ParseError, tk.span.clone(), "error parsing this float literal")),
                };
                return Ok(Expr::new(ExprKind::Float { value }, tk.span.clone()));
            }
            _ => {
                let msg = format!("expected expression, found '{}'", tk.lexeme);
                return Err(throw!(SyntaxError, tk.span.clone(), msg));
            }
        }
    }
}