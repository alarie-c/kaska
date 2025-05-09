use crate::span::Span;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

impl Token {
    /// Just creates an EOF token from the span given
    pub fn eof(span: Span) -> Token {
        Token {
            kind: TokenKind::EOF,
            span,
            lexeme: "<EOF>".to_string(),
        }
    }

    pub fn copy(&self) -> Token {
        Token {
            kind: self.kind,
            span: self.span.clone(),
            lexeme: self.lexeme.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
    EOF = 0,

    // grouping operators
    LParen,
    RParen,
    LCurl,
    RCurl,
    LBrac,
    RBrac,

    Minus,

    // other operators/symbols
    Equal,
    RArrow,
    Colon,
    Semicolon,
    Comma,
    Dot,

    // literals
    True,
    False,
    Ident,
    String,
    Integer,
    Float,

    // keywords
    Def,
    Return,
    Let,
    Mut,
}

impl TokenKind {
    /// Takes a lexeme and eithe returns the keyword corresponding with the lexeme or
    /// identifier in the case that the lexeme has no token kind.
    pub fn from_lexeme(lexeme: &String) -> TokenKind {
        match lexeme.as_str() {
            "def" => TokenKind::Def,
            "return" => TokenKind::Return,
            "let" => TokenKind::Let,
            "mut" => TokenKind::Mut,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Ident,
        }
    }
}
