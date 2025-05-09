use crate::token::{ Token, TokenKind };
use super::{ expr::{ Expr, ExprKind, Operator }, stmt::{ Stmt, StmtKind } };

pub struct Parser {
    pub ast: Vec<Stmt>,
    source: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(source: Vec<Token>) -> Parser {
        Parser { source, ast: vec![], pos: 0 }
    }

    /// Parse the entirety of the source file into a vector of statements
    /// all of which are made of sub expressiosn
    pub fn parse(&mut self) -> &Vec<Stmt> {
        while self.current().kind != TokenKind::EOF {
            if let Some(stmt) = self.parse_stmt() {
                self.ast.push(stmt);
            } else {
                panic!("Got a non expression bro, what did you do...");
            }
            self.advance();
        }

        return &self.ast;
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
        println!("Expected: {:?}, Got: {:?}", next_tk, self.peek().kind);
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
    fn parse_literal(&mut self) -> Option<Expr> {
        let tk = self.current();
        let mut expr: Option<Expr> = None;

        match tk.kind {
            // integer literal
            TokenKind::Integer => {
                let lex = tk.lexeme.replace("_", ""); // remove underscores
                let value = lex.parse::<i32>();
                if value.is_ok() {
                    expr = Some(
                        Expr::new(ExprKind::Integer { value: value.unwrap() }, tk.span.clone())
                    );
                }
            }

            // float literal
            TokenKind::Float => {
                let lex = tk.lexeme.replace("_", ""); // remove underscores
                let value = lex.parse::<f32>();
                if value.is_ok() {
                    expr = Some(
                        Expr::new(ExprKind::Float { value: value.unwrap() }, tk.span.clone())
                    );
                }
            }

            // string literals
            TokenKind::String => {
                expr = Some(
                    Expr::new(ExprKind::String { value: tk.lexeme.to_owned() }, tk.span.clone())
                );
            }

            // identifier literal
            TokenKind::Ident => {
                expr = Some(
                    Expr::new(ExprKind::Ident { name: tk.lexeme.to_owned() }, tk.span.clone())
                );
            }

            // boolean literals
            TokenKind::False => {
                expr = Some(Expr::new(ExprKind::Boolean { value: false }, tk.span.clone()));
            }
            TokenKind::True => {
                expr = Some(Expr::new(ExprKind::Boolean { value: true }, tk.span.clone()));
            }
            _ => {}
        }

        // return expr none or some
        return expr;
    }

    /// Looks for literally any binary operator possible and constructs a binary expression
    /// out of it and the expressions surrounding it.
    fn parse_binary(&mut self) -> Option<Expr> {
        let mut expr = self.parse_literal()?;

        if let Some(op) = Operator::expect_binary(&self.peek().kind) {
            println!("Entered binary");
            self.advance(); // consume the operator
            self.advance(); // go to next expr
            let rhs = self.parse_expr()?;
            let span = expr.span.start..rhs.span.end;
            expr = Expr::new(
                ExprKind::Binary { lhs: Box::new(expr), rhs: Box::new(rhs), op },
                span
            );
        }

        return Some(expr);
    }

    fn parse_assignment(&mut self) -> Option<Expr> {
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

        return Some(expr);
    }

    /// Top level expr parser, just returns the result of parsing any expression starting at `parse_assignment()`
    fn parse_expr(&mut self) -> Option<Expr> {
        return self.parse_assignment();
    }
}

impl Parser {
    /// Stmt parser that parses a variable declaration including optional strong types and let/mut
    /// mutability distinction built into the parser.
    fn parse_variable_decl(&mut self) -> Option<Stmt> {
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
            if let Some(expr) = self.parse_literal() {
                typ = Some(Box::new(expr));
            } else {
                panic!("Expected valid type expression after COLON");
            }
        }

        if !self.expect(TokenKind::Equal) {
            panic!("Expected EQUAL after variable declaration");
        }

        self.advance(); // move to beginning of value expr;
        match self.parse_expr() {
            Some(expr) => {
                let span = tk.span.start..expr.span.end;
                return Some(
                    Stmt::new(
                        StmtKind::VariableDecl {
                            mutable,
                            name,
                            value: Box::new(expr),
                            typ,
                        },
                        span
                    )
                );
            }
            None => panic!("Expected a valid value after variable decl"),
        }
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
    fn parse_stmt(&mut self) -> Option<Stmt> {
        let tk = self.current();
        let mut stmt: Option<Stmt> = None;

        match tk.kind {
            TokenKind::Let | TokenKind::Mut => {
                stmt = self.parse_variable_decl();
            }

            // match on expression statements
            _ => if let Some(expr) = self.parse_expr() {
                match expr.kind {
                    ExprKind::Assignment { assignee: _, value: _, op: _ } => {
                        let span = expr.span.clone();
                        stmt = Some(Stmt::new(StmtKind::Expression { expr: Box::new(expr) }, span));
                    }
                    _ => panic!("..."),
                }
            }
        }

        if !self.expect(TokenKind::Semicolon) {
            panic!("Expected a SEMICOLON after statement");
        }
        return stmt;
    }
}
