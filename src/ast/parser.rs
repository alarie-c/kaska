use crate::{errors::{Error, ErrorBuffer, ErrorKind}, token::{ Token, TokenKind }};
use super::{ expr::{ Expr, ExprKind, Operator }, stmt::{ Stmt, StmtKind } };

pub type AST = Vec<Stmt>;

pub struct Parser {
    source: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(source: Vec<Token>) -> Parser {
        Parser { source, pos: 0 }
    }

    /// Parse the entirety of the source file into a vector of statements
    /// all of which are made of sub expressiosn
    pub fn parse(&mut self) -> (AST, ErrorBuffer) {
        let mut errors: ErrorBuffer = vec![];
        let mut ast: AST = vec![];
        
        while self.current().kind != TokenKind::EOF {
            match self.parse_stmt() {
                Ok(stmt) => ast.push(stmt),
                Err(e) => errors.push(e),
            }

            // Consume the semicolon that should follow
            if !self.expect(TokenKind::Semicolon) {
                errors.push(Error::new(ErrorKind::SyntaxError, self.current().span.clone(), "expected semicolon to end statement", true));
            }

            self.advance();
        }

        return (ast, errors);
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

        return Err(Error::new(ErrorKind::ParseError, tk.span.clone(), "expected a primary expression", true));
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
        return self.parse_assignment();
    }
}

impl Parser {
    /// Stmt parser that parses a variable declaration including optional strong types and let/mut
    /// mutability distinction built into the parser.
    fn parse_variable_decl(&mut self) -> Result<Stmt, Error> {
        let tk = self.current();

        // set mutability based on leading token keyword
        let mutable = tk.kind == TokenKind::Mut;

        self.advance();
        if self.current().kind != TokenKind::Ident {
            panic!("Expected a name after variable decl");
        }

        let name = self.current().lexeme.to_owned();
        let mut typ: Option<Box<Expr>> = None;

        // get strong type after colon
        if self.expect(TokenKind::Colon) {
            self.advance(); // go to start of type expr
            if let Ok(expr) = self.parse_literal() {
                typ = Some(Box::new(expr));
            } else {
                return Err(Error::new(ErrorKind::SyntaxError, self.current().span.clone(), "expected type name", true));
            }
        }

        if !self.expect(TokenKind::Equal) {
            return Err(Error::new(ErrorKind::SyntaxError, self.current().span.clone(), "expected value after variable definition", true));
        }

        self.advance(); // move to beginning of value expr;
        let value = self.parse_expr()?;
        let span = tk.span.start..value.span.end;
        return Ok(
            Stmt::new(
                StmtKind::VariableDecl {
                    mutable,
                    name,
                    value: Box::new(value),
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
        let tk = self.current();

        match tk.kind {
            TokenKind::Let | TokenKind::Mut => self.parse_variable_decl(),

            // match on expression statements
            _ => {
                let expr = self.parse_expr()?;
                match expr.kind {
                    ExprKind::Assignment { assignee: _, value: _, op: _ } => {
                        let span = expr.span.clone();
                        return Ok(Stmt::new(StmtKind::Expression { expr: Box::new(expr) }, span));
                    }
                    _ => {},
                }
                return Err(Error::new(ErrorKind::SyntaxError, tk.span.clone(), "expressions must be meaningful to exit on their own", true));
            }
        }
    }
}
