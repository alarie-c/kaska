use std::io::empty;

use crate::{ common::errors::{ Error, ErrorBuffer, ErrorKind }, expr, stmt, throw };
use super::{ ast::{ Expr, ExprKind, Operator, Stmt, StmtKind }, lexer::{ Token, TokenKind } };

// ----------------------------------------------------------------- \\
// HELPER PARSERS
// ----------------------------------------------------------------- \\

fn parse_integer(tk: &Token) -> Result<Expr, Error> {
    let value = match tk.lexeme.parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            return Err(throw!(ParseError, tk.span.clone(), "error parsing integer literal"));
        }
    };
    return Ok(Expr::new(ExprKind::Integer { value }, tk.span.clone()));
}

fn parse_float(tk: &Token) -> Result<Expr, Error> {
    let value = match tk.lexeme.parse::<f32>() {
        Ok(value) => value,
        Err(_) => {
            return Err(throw!(ParseError, tk.span.clone(), "error parsing float literal"));
        }
    };
    return Ok(Expr::new(ExprKind::Float { value }, tk.span.clone()));
}

fn parse_ident(tk: &Token) -> Result<Expr, Error> {
    let name = tk.lexeme.clone();
    return Ok(Expr::new(ExprKind::Ident { name }, tk.span.clone()));
}

fn parse_string(tk: &Token) -> Result<Expr, Error> {
    let value = tk.lexeme.clone();
    return Ok(Expr::new(ExprKind::String { value }, tk.span.clone()));
}

// ----------------------------------------------------------------- \\
// PARSER IMPLEMENTATION
// ----------------------------------------------------------------- \\

pub struct Parser {
    pub errors: Vec<Error>,
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        return Parser {
            errors: vec![],
            tokens,
            pos: 0,
        };
    }

    pub fn parse(&mut self) -> (Vec<Stmt>, ErrorBuffer) {
        let ast: Vec<Stmt> = self.parse_program();
        let ebuffer = self.errors.drain(0..).collect();
        return (ast, ebuffer);
    }
}

impl Parser {
    fn at_end(&self) -> bool {
        return self.current().kind == TokenKind::EOF;
    }

    /// Returns whatever is at the current position of the parser.
    fn current(&self) -> &Token {
        return self.tokens.get(self.pos).unwrap_or(&self.tokens[self.tokens.len() - 1]);
    }

    /// Returns the next thing past the current position of the parser without changing the state of the parser.
    fn peek(&self) -> &Token {
        return self.tokens.get(self.pos + 1).unwrap_or(&self.tokens[self.tokens.len() - 1]);
    }

    /// Moves the position of the parser forward but will never exceed the EOF index.
    fn consume(&mut self) {
        self.pos = (self.pos + 1).clamp(0, self.tokens.len() - 1);
    }

    /// Expects the next token to be of the prescribed type and consume if it is. If it isn't, it will not consume but it will return an error.
    fn assert_next(&mut self, kind: TokenKind, msg: String) -> Result<(), Error> {
        if !self.expect_next(kind) {
            let span = self.peek().span.clone();
            return Err(throw!(SyntaxError, span, msg));
        }
        return Ok(());
    }

    /// Asserts that the current token should be of the type and throws an error otherwise
    fn assert_current(&mut self, kind: TokenKind, msg: String) -> Result<(), Error> {
        if self.current().kind != kind {
            let span = self.current().span.clone();
            return Err(throw!(SyntaxError, span, msg));
        }
        return Ok(());
    }

    /// Same as assert but ignores newlines
    fn assert_next_ignore_newln(&mut self, kind: TokenKind, msg: &str) -> Result<(), Error> {
        if !self.expect_next_ignore_newln(kind) {
            let span = self.peek().span.clone();
            return Err(throw!(SyntaxError, span, msg.to_string()));
        }
        return Ok(());
    }

    /// Returns whether or not the next token is of the prescribed type and consumes if it is
    fn expect_next(&mut self, kind: TokenKind) -> bool {
        if self.peek().kind == kind {
            self.consume();
            return true;
        }
        return false;
    }

    /// Returns whether or not the current token is of the prescribed type
    fn expect_current(&self, kind: TokenKind) -> bool {
        return self.current().kind == kind;
    }

    /// Identical to expect but ignores newlines
    fn expect_next_ignore_newln(&mut self, kind: TokenKind) -> bool {
        self.skip_next_newlines();
        return self.expect_next(kind);
    }

    /// Keeps advancing until the next token is NOT a newline
    fn skip_next_newlines(&mut self) {
        while self.peek().kind == TokenKind::Newline {
            self.consume();
        }
    }

    /// Keeps advancing until the current token is NOT a newline
    fn skip_newlines(&mut self) {
        while self.current().kind == TokenKind::Newline {
            self.consume();
        }
    }

    /// Starts from the current thing and then looks for something to end the
    /// current line and start a new statment.
    ///
    /// This function will end on the semicolon/newline, NOT on the first
    /// thing of the next line, so call `self.advance()`.
    fn sync(&mut self) {
        while !self.at_end() {
            match self.current().kind {
                TokenKind::EOF | TokenKind::Semicolon | TokenKind::Newline => {
                    break;
                }
                _ => {}
            }
            self.consume();
        }
    }

    /// Similar to `sync` but will look for the END token instead
    fn sync_after_fn(&mut self) {
        while !self.at_end() {
            match self.current().kind {
                TokenKind::EOF | TokenKind::End => {
                    break;
                }
                _ => {}
            }
            self.consume();
        }
    }

    fn emit_diagnostics(&self, location: &str) {
        println!("\nLOCATED: {}", location);
        println!("CURRENT: {:?}", self.current().kind);
        println!("NEXT: {:?}", self.peek().kind);
    }

    fn err(&mut self, error: Error) {
        self.errors.push(error);
    }
}

// ----------------------------------------------------------------- \\
// HELPER PARSERS
// ----------------------------------------------------------------- \\

impl Parser {
    fn parse_args(&mut self) -> Result<Vec<Expr>, Error> {
        let mut args = Vec::<Expr>::new();
        self.consume();

        // start: first expr of arg 1
        loop {
            if self.current().kind == TokenKind::RParen {
                break;
            }

            let expr = self.expr()?;
            args.push(expr);

            // next is either COMMA or RPAREN
            if self.expect_next_ignore_newln(TokenKind::Comma) {
                self.consume();
                continue;
            } else {
                self.assert_next_ignore_newln(
                    TokenKind::RParen,
                    "expected ')' to close function call arguments"
                )?;
                break;
            }
        }

        // end: RPAREN
        return Ok(args);
    }

    fn parse_params(&mut self) -> Result<Vec<Expr>, Error> {
        let mut params = Vec::<Expr>::new();

        // start: first expr of param 1
        loop {
            if self.current().kind == TokenKind::RParen {
                break;
            }

            self.assert_current(TokenKind::Ident, "expected parameter name".to_string())?;
            let name = self.current().lexeme.clone();
            let start = self.current().span.start;

            self.assert_next(
                TokenKind::Colon,
                format!("expected colon after parameter name, got '{}'", self.current().lexeme)
            )?;
            self.consume(); // go to start of type expression

            let typ = self.expr()?;
            let span = start..typ.span.end;
            params.push(expr!(Parameter, name, typ, span));

            // next is either COMMA or RPAREN
            if self.expect_next_ignore_newln(TokenKind::Comma) {
                self.consume();
                continue;
            } else {
                self.assert_next_ignore_newln(
                    TokenKind::RParen,
                    "expected ')' to close function parameters"
                )?;
                break;
            }
        }

        // end: RPAREN
        return Ok(params);
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        let mut stmts: Vec<Stmt> = vec![];

        while !self.at_end() {
            self.skip_newlines();
            if self.expect_current(TokenKind::End) {
                return stmts;
            }

            match self.stmt() {
                Ok(stmt) => stmts.push(stmt),
                Err(err) => {
                    self.err(err);
                    self.sync();
                }
            }

            self.consume();
        }

        // this only happens if there's no END to close
        // throw error and return so semantic analysis can happen on this block
        self.err(
            throw!(SyntaxError, self.current().span.clone(), "block is missing 'end' delimiter")
        );
        return stmts;
    }
}

// ----------------------------------------------------------------- \\
// EXPRESSION PARSERS
// ----------------------------------------------------------------- \\

impl Parser {
    fn expr_literal(&mut self) -> Result<Expr, Error> {
        let tk = self.current();
        let span = tk.span.clone();

        match &tk.kind {
            TokenKind::Integer => parse_integer(tk),
            TokenKind::Float => parse_float(tk),
            TokenKind::Ident => parse_ident(tk),
            TokenKind::String => parse_string(tk),

            TokenKind::True | TokenKind::False => {
                let value = tk.kind == TokenKind::True;
                return Ok(Expr::new(ExprKind::Boolean { value }, span));
            }

            _ => {
                return Err(
                    throw!(SyntaxError, span, format!("expected expression, got '{}'", tk.lexeme))
                );
            }
        }
    }

    fn expr_call(&mut self) -> Result<Expr, Error> {
        let mut expr = self.expr_literal()?;

        if self.expect_next(TokenKind::LParen) {
            self.skip_newlines();
            let args = self.parse_args()?;
            let span = expr.span.start..self.current().span.end;
            expr = expr!(Call, expr, args, span);
        }

        return Ok(expr);
    }

    fn expr_binary(&mut self) -> Result<Expr, Error> {
        let mut expr = self.expr_call()?;

        if let Some(op) = Operator::binary(&self.peek().kind) {
            self.consume(); // consume the operator
            self.consume(); // go to start of next expr
            self.skip_newlines();
            let rhs = self.expr()?;
            let span = expr.span.start..rhs.span.end;
            expr = expr!(Binary, expr, rhs, op, span);
        }

        return Ok(expr);
    }

    fn expr_assignment(&mut self) -> Result<Expr, Error> {
        let mut expr = self.expr_binary()?;

        if let Some(op) = Operator::assignment(&self.peek().kind) {
            self.consume(); // consume the operator
            self.consume(); // go to start of next expr
            self.skip_newlines();
            let value = self.expr()?;
            let span = expr.span.start..value.span.end;
            expr = expr!(Binary, expr, value, op, span);
        }

        return Ok(expr);
    }

    fn expr(&mut self) -> Result<Expr, Error> {
        return self.expr_assignment();
    }
}

// ----------------------------------------------------------------- \\
// STMT PARSERS
// ----------------------------------------------------------------- \\

impl Parser {
    fn stmt_function(&mut self) -> Result<Stmt, Error> {
        self.assert_next(
            TokenKind::Ident,
            format!("expected variable name, got {}", self.peek().lexeme)
        )?;

        // start: IDENT
        let name = self.current().lexeme.clone();
        let start = self.current().span.start;

        self.assert_next(
            TokenKind::LParen,
            "expected '(' to begin function parameters".to_string()
        )?;
        self.consume();

        // get the parameters
        let params = self.parse_params()?;

        // get the return type
        let ret: Option<Expr> = if self.expect_next_ignore_newln(TokenKind::RArrow) {
            self.consume(); // move to start of type expression
            Some(self.expr()?)
        } else {
            None
        };

        self.consume();

        self.skip_newlines();
        let body = self.parse_block();

        // end: END
        let span = start..self.current().span.end;
        return Ok(stmt!(Function, name, ret, params, body, span));
    }

    fn stmt_variable(&mut self) -> Result<Stmt, Error> {
        self.assert_next(
            TokenKind::Ident,
            format!("expected variable name, got {}", self.peek().lexeme)
        )?;

        // start: IDENT
        let name = self.current().lexeme.clone();
        let start = self.current().span.start;

        let mut typ: Option<Expr> = None;
        if self.expect_next(TokenKind::Colon) {
            self.consume(); // go to start of expr
            typ = Some(self.expr_literal()?);
        }

        self.assert_next(TokenKind::Equal, format!("expected '=', got {}", self.peek().lexeme))?;

        self.skip_next_newlines();
        self.consume(); // go to start of value
        let value = self.expr()?;

        let span = start..value.span.end;
        return Ok(stmt!(Variable, name, typ, value, span));
    }

    fn stmt(&mut self) -> Result<Stmt, Error> {
        self.emit_diagnostics("stmt");
        self.skip_newlines();

        let stmt: Stmt = match &self.current().kind {
            TokenKind::Let => self.stmt_variable()?,
            _ => {
                self.emit_diagnostics("stmt parser");
                unimplemented!()
            }
        };

        // look for end of stmt
        let next = self.peek().kind;
        if next != TokenKind::Semicolon && next != TokenKind::Newline && next != TokenKind::EOF {
            return Err(
                throw!(
                    SyntaxError,
                    self.current().span.clone(),
                    "expected ';' or new line to complete statement"
                )
            );
        }
        self.consume();

        return Ok(stmt);
    }
}

// ----------------------------------------------------------------- \\
// TOP-LEVEL PARSER
// ----------------------------------------------------------------- \\

impl Parser {
    fn parse_program(&mut self) -> Vec<Stmt> {
        let mut stmts: Vec<Stmt> = vec![];

        while !self.at_end() {
            match &self.current().kind {
                TokenKind::Function =>
                    match self.stmt_function() {
                        Ok(stmt) => stmts.push(stmt),
                        Err(err) => {
                            self.err(err);
                            self.sync_after_fn();
                        }
                    }
                _ => {
                    self.emit_diagnostics("parse program");
                    unimplemented!();
                }
            }

            self.consume();
            self.skip_newlines();
        }

        return stmts;
    }
}
