use crate::common::span::Span;

/// Alias for TokenKind
pub type Tk = TokenKind;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

impl Token {
    /// A little helper function to make token construction a little easier
    pub fn new(kind: TokenKind, span: Span, lexeme: &str) -> Token {
        return Token {
            kind,
            span,
            lexeme: lexeme.to_string(),
        };
    }

    /// Just creates an EOF token from the span given
    pub fn eof(span: Span) -> Token {
        Token {
            kind: TokenKind::EOF,
            span,
            lexeme: "<EOF>".to_string(),
        }
    }

    // /// Self explanatory, Rust won't let me just implement the trait
    // pub fn copy(&self) -> Token {
    //     Token {
    //         kind: self.kind,
    //         span: self.span.clone(),
    //         lexeme: self.lexeme.to_owned(),
    //     }
    // }
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

    // arithmetic operators
    Plus,
    PlusPlus,
    PlusEqual,
    Minus,
    MinusMinus,
    MinusEqual,
    Star,
    StarStar,
    StarEqual,
    StarStarEqual,
    Slash,
    SlashSlash,
    SlashEqual,
    SlashSlashEqual,
    Modulo,

    // comparison operators
    Less,
    LessEqual,
    More,
    MoreEqual,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,

    // logical operators
    Pipe,
    PipePipe,
    Amprsnd,
    AmprsndAmprsnd,

    // other operators/symbols
    RArrow,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Newline,
    Sigil,
    Ellipsis,

    // literals
    True,
    False,
    Ident,
    String,
    Integer,
    Float,

    // keywords
    Let,
    Function,
    Return,
    If,
    Else,
    For,
    While,
    Break,
    Class,
    Enum,
    End,
    Where,
    Is,
    Not,
    Import,
    From,
    As,
    Inline,
    Pub,
    In,
}

impl TokenKind {
    /// Takes a lexeme and eithe returns the keyword corresponding with the lexeme or
    /// identifier in the case that the lexeme has no token kind.
    pub fn from_lexeme(lexeme: &String) -> TokenKind {
        match lexeme.as_str() {
            "let" => TokenKind::Let,
            "function" => TokenKind::Function,
            "return" => TokenKind::Return,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "break" => TokenKind::Break,
            "class" => TokenKind::Class,
            "enum" => TokenKind::Enum,
            "end" => TokenKind::End,
            "is" => TokenKind::Is,
            "not" => TokenKind::Not,
            "where" => TokenKind::Where,
            "import" => TokenKind::Import,
            "from" => TokenKind::From,
            "as" => TokenKind::As,
            "inline" => TokenKind::Inline,
            "pub" => TokenKind::Pub,
            "in" => TokenKind::In,
            _ => TokenKind::Ident,
        }
    }
}
