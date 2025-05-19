use crate::common::errors::{ Error, ErrorBuffer, ErrorKind };
use super::{ ast::{ Expr, ExprKind, Operator, Stmt, StmtKind }, lexer::{ Token, TokenKind } };

pub type AST = Vec<Stmt>;

/// High level interface for the caller of this function,
/// basically allowed streamlined snatching of the error buffer from
/// the parser before returning, that way the error buffer is owned
/// when returned.
pub fn parse(source: Vec<Token>) -> (AST, ErrorBuffer) {
    let mut parser = Parser::new(source);
    let ast = parser.parse();

    let mut errors: ErrorBuffer = vec![];
    let _ = std::mem::replace(&mut errors, parser.errors);
    return (ast, errors);
}

pub struct Parser {
    errors: ErrorBuffer,
    source: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(source: Vec<Token>) -> Parser {
        Parser { errors: vec![], source, pos: 0 }
    }

    /// Parse the entirety of the source file into a vector of statements
    /// all of which are made of sub expressiosn
    pub fn parse(&mut self) -> AST {
        let mut ast: AST = vec![];

        while self.current().kind != TokenKind::EOF {
            match self.parse_stmt() {
                Ok(stmt) => ast.push(stmt),
                Err(e) => self.errors.push(e),
            }
            self.advance();
        }

        return ast;
    }
}

impl Parser {
    /// Returns a copied version of the token at the current position <=> pos is not at end
    fn current(&self) -> Token {
        if self.source.len() > self.pos {
            return self.source[self.pos].copy();
        }
        return self.source.last().unwrap().copy();
    }

    /// Returns a copied version of the token at the current position + 1 <=> `pos + 1` is not at end
    fn peek(&self) -> Token {
        if self.source.len() > self.pos + 1 {
            return self.source[self.pos + 1].copy();
        }
        return self.source.last().unwrap().copy();
    }

    /// Expect a peeked token and then assert whether or not it is equivalent to the type passed
    /// in the parameter `next_tk`
    fn expect(&mut self, next_tk: TokenKind) -> bool {
        if self.peek().kind == next_tk {
            self.advance();
            return true;
        }
        return false;
    }

    /// Basicalyl exactly the same as expect but returns a syntax error if the token isn't found
    fn assert_err(&mut self, next_tk: TokenKind, msg: &str) -> Result<(), Error> {
        if self.peek().kind == next_tk {
            self.advance();
            return Ok(());
        }
        return Err(Error::new(ErrorKind::SyntaxError, self.current().span.clone(), msg, true));
    }

    /// Advances a token and pushes a syntax error if it wasn't what was expected
    fn assert(&mut self, next_tk: TokenKind, msg: &str) {
        if self.peek().kind == next_tk {
            self.advance();
            return;
        }
        self.errors.push(
            Error::new(ErrorKind::SyntaxError, self.current().span.clone(), msg, true)
        );
        self.advance();
    }

    fn assert_end(&mut self) {
        if self.peek().kind == TokenKind::Semicolon
        || self.peek().kind == TokenKind::Newline
        || self.peek().kind == TokenKind::EOF {
            self.advance();
            return;
        }
        self.errors.push(
            Error::new(ErrorKind::SyntaxError, self.current().span.clone(), "expected ';' or new line", true)
        );
        self.advance();
    }

    /// Move the position one step ahead <=> pos is not at end
    fn advance(&mut self) {
        if self.source.len() <= self.pos + 1 {
            return;
        }
        self.pos += 1;
    }
}

impl Parser {
    /// Expr parser that parses any and all types of literals including
    /// * integers
    /// * floats
    /// * strings
    /// * booleans
    /// * identifiers
    /// * (eventually) arrays/matrices/quaternions/ etc.
    fn parse_literal(&mut self) -> Result<Expr, Error> {
        let tk = self.current();

        match tk.kind {
            // integer literal
            TokenKind::Integer => {
                let lex = tk.lexeme.replace("_", ""); // remove underscores
                let value = lex.parse::<i32>();
                if value.is_ok() {
                    return Ok(
                        Expr::new(ExprKind::Integer { value: value.unwrap() }, tk.span.clone())
                    );
                }
            }

            // float literal
            TokenKind::Float => {
                let lex = tk.lexeme.replace("_", ""); // remove underscores
                let value = lex.parse::<f32>();
                if value.is_ok() {
                    return Ok(
                        Expr::new(ExprKind::Float { value: value.unwrap() }, tk.span.clone())
                    );
                }
            }

            // string literals
            TokenKind::String => {
                return Ok(
                    Expr::new(ExprKind::String { value: tk.lexeme.to_owned() }, tk.span.clone())
                );
            }

            // identifier literal
            TokenKind::Ident => {
                return Ok(
                    Expr::new(ExprKind::Ident { name: tk.lexeme.to_owned() }, tk.span.clone())
                );
            }

            // boolean literals
            TokenKind::False => {
                return Ok(Expr::new(ExprKind::Boolean { value: false }, tk.span.clone()));
            }
            TokenKind::True => {
                return Ok(Expr::new(ExprKind::Boolean { value: true }, tk.span.clone()));
            }
            _ => {}
        }

        return Err(
            Error::new(
                ErrorKind::SyntaxError,
                tk.span.clone(),
                "expected a primary expression",
                true
            )
        );
    }

    /// Looks for literally any binary operator possible and constructs a binary expression
    /// out of it and the expressions surrounding it.
    fn parse_binary(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_literal()?;

        if let Some(op) = Operator::expect_binary(&self.peek().kind) {
            self.advance(); // consume the operator
            self.advance(); // go to next expr
            let rhs = self.parse_expr()?;
            let span = expr.span.start..rhs.span.end;
            expr = Expr::new(
                ExprKind::Binary { lhs: Box::new(expr), rhs: Box::new(rhs), op },
                span
            );
        }

        return Ok(expr);
    }

    fn parse_assignment(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_binary()?;

        if let Some(op) = Operator::expect_assign(&self.peek().kind) {
            self.advance(); // consume the operator
            self.advance(); // go to the next expr
            let value = self.parse_expr()?;
            let span = expr.span.start..value.span.end;
            expr = Expr::new(
                ExprKind::Assignment { assignee: Box::new(expr), value: Box::new(value), op },
                span
            );
        }

        return Ok(expr);
    }

    /// Top level expr parser, just returns the result of parsing any expression starting at `parse_assignment()`
    fn parse_expr(&mut self) -> Result<Expr, Error> {
        let e = self.parse_assignment();
        return e;
    }
}

impl Parser {
    fn parse_proc_decl(&mut self) -> Result<Stmt, Error> {
        let tk = self.current();

        // get the name
        self.assert(TokenKind::Ident, "expected procedure name after 'def'");
        let name = self.current().lexeme.to_owned();

        // TODO: generic implementation `proc<T>()`

        // begin parameters
        self.assert(TokenKind::LParen, "expected '(' after procedure name");
        self.advance(); // skip the lparen now
        let mut params = Vec::<Expr>::new();

        // gather up all the parameters
        if self.current().kind != TokenKind::RParen {
            self.pos -= 1;
            loop {
                // name
                self.assert(TokenKind::Ident, "expected parameter name");
                let span_start = self.current().span.start;
                let name = self.current().lexeme.to_owned();

                // type
                self.assert(TokenKind::Colon, "expected type after parameter name");
                self.advance(); // skip the colon
                let ty = self.parse_expr()?;
                let span_end = ty.span.end;

                params.push(
                    Expr::new(ExprKind::Parameter { name, ty: Box::new(ty) }, span_start..span_end)
                );

                if self.expect(TokenKind::Comma) {
                    continue;
                } else {
                    self.advance();
                    break;
                }
            }
        }

        // close out the parameters
        if self.current().kind != TokenKind::RParen {
            return Err(
                Error::new(
                    ErrorKind::SyntaxError,
                    self.current().span.clone(),
                    "expected ')' to close procedure parameters",
                    true
                )
            );
        }

        // get the return type
        let mut returns: Option<Expr> = None;
        if self.expect(TokenKind::RArrow) {
            self.advance(); // skip the arrow
            returns = Some(self.parse_expr()?);
        }

        // start getting the function body
        let mut body = Vec::<Stmt>::new();
        self.assert_err(TokenKind::Colon, "expected procedure body")?;
        self.advance(); // skip the colon

        // gather up the stmts
        loop {
            if self.current().kind == TokenKind::End {
                break;
            }

            if self.current().kind == TokenKind::EOF {
                self.errors.push(
                    Error::new(
                        ErrorKind::SyntaxError,
                        self.current().span.clone(),
                        "procedure body is missing a closing 'end'",
                        true
                    )
                );
                break;
            }

            match self.parse_stmt() {
                Ok(stmt) => body.push(stmt),
                Err(e) => self.errors.push(e),
            }
            self.advance();
        }

        let span_end = self.current().span.end;
        return Ok(
            Stmt::new(StmtKind::ProcDef { name, params, returns, body }, tk.span.start..span_end)
        );
    }

    /// Stmt parser that parses a variable declaration including optional strong types and let/mut
    /// mutability distinction built into the parser.
    fn parse_variable_decl(&mut self) -> Result<Stmt, Error> {
        let tk = self.current();
        let name = self.current().lexeme.to_owned();
        let mut typ: Option<Expr> = None;
        self.advance(); // move to the colon

        // get strong type after colon
        match self.peek().kind {
            TokenKind::Ident => {
                self.advance(); // consume ident
                typ = Some(self.parse_literal()?);
                self.assert_err(TokenKind::Equal, "expected '=' after type name")?;
            },
            TokenKind::Equal => {
                self.advance();
            }
            _ => {
                return Err(Error::new(ErrorKind::SyntaxError, self.peek().span.clone(), "expected a type name or '=' after variable declaration", true));
            }
        }

        self.advance(); // move to beginning of value expr;
        let value = self.parse_expr()?;
        let span = tk.span.start..value.span.end;
        return Ok(
            Stmt::new(
                StmtKind::VarDecl {
                    name,
                    value,
                    typ,
                },
                span
            )
        );
    }

    /// Top level stmt parser, determines which stmt parser to use based on the keyword at the beginning of the line.
    /// If no valid keyword can be found, will create an expression statement instead and assert that the expression
    /// can hold meaning on it's own.
    /// * variable decl
    /// * procedure decl
    /// * return stmt
    /// * for loops
    /// * while loops
    /// * if/else
    fn parse_stmt(&mut self) -> Result<Stmt, Error> {
        while self.current().kind == TokenKind::Newline {
            self.advance();
        }
        
        let tk = self.current();
        let stmt: Stmt;

        if tk.kind == TokenKind::Ident && self.peek().kind == TokenKind::Colon {
            stmt = self.parse_variable_decl()?;
        } else if tk.kind == TokenKind::Def {
            stmt = self.parse_proc_decl()?;
        } else if tk.kind == TokenKind::Return {
            self.advance(); // consume RETURN
            let expr = self.parse_expr()?;
            let span = expr.span.clone();
            stmt = Stmt::new(StmtKind::Return { expr }, span);
        } else {
            let expr = self.parse_expr()?;
            match expr.kind {
                ExprKind::Assignment { assignee: _, value: _, op: _ } => {
                    let span = expr.span.clone();
                    stmt = Stmt::new(StmtKind::Expr { expr }, span);
                }
                _ => {
                    return Err(
                        Error::new(
                            ErrorKind::SyntaxError,
                            tk.span.clone(),
                            "expressions must be meaningful to exist on their own",
                            true
                        )
                    );
                }
            } 
        }

        // consume the semicolon
        self.assert_end();
        return Ok(stmt);
    }
}
