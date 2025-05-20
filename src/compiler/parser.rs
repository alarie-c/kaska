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

    pub fn parse(&mut self) -> (Vec<Stmt>, ErrorBuffer) {
        let mut ebuffer: ErrorBuffer = vec![];
        let mut ast: Vec<Stmt> = vec![];

        while self.current().kind != TokenKind::EOF {
            match self.stmt() {
                Ok(stmt) => ast.push(stmt),
                Err(err) => ebuffer.push(err),
            }
            self.consume();
        }

        return (ast, ebuffer);
    }
}

impl Parser {
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
    fn assert(&mut self, kind: TokenKind, msg: String) -> Result<(), Error> {
        if !self.expect(kind) {
            let span = self.current().span.clone();
            return Err(throw!(SyntaxError, span, msg));
        }
        return Ok(());
    }

    /// Same as assert but ignores newlines
    fn assert_ignore_newln(&mut self, kind: TokenKind, msg: &str) -> Result<(), Error> {
        if !self.expect_ignore_newln(kind) {
            let span = self.current().span.clone();
            return Err(throw!(SyntaxError, span, msg.to_string()));
        }
        return Ok(());
    }

    /// Returns whether or not the next token is of the prescribed type and consumes if it is
    fn expect(&mut self, kind: TokenKind) -> bool {
        if self.peek().kind == kind {
            self.consume();
            return true;
        }
        return false;
    }

    /// Identical to expect but ignores newlines
    fn expect_ignore_newln(&mut self, kind: TokenKind) -> bool {
        self.skip_next_newlines();
        return self.expect(kind);
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

    fn emit_diagnostics(&self, location: &str) {
        println!("\nLOCATED: {}", location);
        println!("CURRENT: {:?}", self.current().kind);
        println!("NEXT: {:?}", self.peek().kind);
    }
}

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
            if self.expect_ignore_newln(TokenKind::Comma) {
                self.consume();
                continue;
            } else {
                self.assert_ignore_newln(
                    TokenKind::RParen,
                    "expected ')' to close function call arguments"
                )?;
                break;
            }
        }

        // end: RPAREN
        return Ok(args);
    }
}

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

        if self.expect(TokenKind::LParen) {
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

impl Parser {
    fn stmt_variable(&mut self) -> Result<Stmt, Error> {
        // start: IDENT
        let name = self.current().lexeme.clone();
        let start = self.current().span.start;

        let mut typ: Option<Expr> = None;
        if self.expect(TokenKind::Colon) {
            self.consume(); // go to start of expr
            typ = Some(self.expr_literal()?);
        }

        self.assert(TokenKind::Equal, format!("expected '=', got {}", self.peek().lexeme))?;

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
            TokenKind::Let => {
                self.assert(
                    TokenKind::Ident,
                    format!("expected variable name, got {}", self.peek().lexeme)
                )?;

                self.stmt_variable()?
            }
            _ => unimplemented!(),
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

        return Ok(stmt);
    }
}
