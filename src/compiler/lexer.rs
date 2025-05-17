use crate::common::{ errors::{ Error, ErrorBuffer, ErrorKind }, span::Span };

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

impl Token {
    /// A little helper function to make token construction a little easier
    fn new(kind: TokenKind, span: Span, lexeme: &str) -> Token {
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

    /// Self explanatory, Rust won't let me just implement the trait
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
    Plus,

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

pub struct Lexer<'a> {
    source: &'a String,
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Initializes a new lexer with the given source
    pub fn new(source: &'a String) -> Lexer<'a> {
        Lexer {
            source,
            pos: 0,
        }
    }

    /// Takes the input given to the lexer and iterates through, creating tokens
    /// and eventually returning them as a vector
    pub fn lex(&mut self) -> (Vec<Token>, ErrorBuffer) {
        let mut tokens = Vec::<Token>::new();
        let mut errors: ErrorBuffer = vec![];

        while let Some(ch) = self.current() {
            let start = self.pos;

            match ch {
                // whitespace ignore
                ' ' | '\t' | '\r' | '\n' => {}

                // grouping operators
                '(' => tokens.push(Token::new(TokenKind::LParen, start..start, "(")),
                ')' => tokens.push(Token::new(TokenKind::RParen, start..start, ")")),
                '[' => tokens.push(Token::new(TokenKind::LBrac, start..start, "[")),
                ']' => tokens.push(Token::new(TokenKind::RBrac, start..start, "]")),
                '{' => tokens.push(Token::new(TokenKind::LCurl, start..start, "{")),
                '}' => tokens.push(Token::new(TokenKind::RCurl, start..start, "}")),

                '=' => tokens.push(Token::new(TokenKind::Equal, start..start, "=")),

                // other operators/symbols
                '-' => if self.expect('>') {
                    tokens.push(Token::new(TokenKind::RArrow, start..self.pos, "->"));
                } else {
                    tokens.push(Token::new(TokenKind::Minus, start..self.pos, "-"));
                }

                '+' => tokens.push(Token::new(TokenKind::Plus, start..start, "+")),

                ':' => tokens.push(Token::new(TokenKind::Colon, start..start, ":")),
                ';' => tokens.push(Token::new(TokenKind::Semicolon, start..start, ";")),
                ',' => tokens.push(Token::new(TokenKind::Comma, start..start, ",")),
                '.' => tokens.push(Token::new(TokenKind::Dot, start..start, ".")),

                '"' => {
                    let mut lexeme = String::new();

                    // consume while valid identifier
                    'string: loop {
                        match self.peek() {
                            // if a valid cahracter comes next
                            Some(next_ch) => if next_ch == '\\' {
                                panic!("Escape sequences not support yet brochacho... </3");
                            } else if next_ch == '"' {
                                self.advance();
                                break 'string;
                            } else {
                                self.advance();
                                lexeme.push(next_ch);
                            }

                            // uh oh cherio
                            None => {
                                errors.push(
                                    Error::new(
                                        ErrorKind::SyntaxError,
                                        start..self.pos,
                                        "string literal is missing a closing '\"'",
                                        true
                                    )
                                );
                                break;
                            }
                        }
                    }

                    tokens.push(Token {
                        kind: TokenKind::String,
                        span: start..self.pos,
                        lexeme,
                    });
                }

                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut lexeme = String::from(ch);

                    // consume while valid identifier
                    while let Some(next_ch) = self.peek() {
                        if !next_ch.is_alphanumeric() && next_ch != '_' {
                            break;
                        }
                        self.advance();
                        lexeme.push(next_ch);
                    }

                    let kind = TokenKind::from_lexeme(&lexeme);
                    tokens.push(Token { kind, span: start..self.pos, lexeme });
                }

                '0'..='9' => {
                    let mut lexeme = String::from(ch);
                    let mut kind = TokenKind::Integer;

                    // consume while valid number
                    while let Some(next_ch) = self.peek() {
                        if !next_ch.is_ascii_digit() && next_ch != '_' && next_ch != '.' {
                            break;
                        }

                        // update token kind when first decimal encountered
                        if next_ch == '.' {
                            if kind == TokenKind::Float {
                                break; // allow for decimals index into number literal
                            }

                            self.advance();
                            if let Some(after_decimal) = self.peek() {
                                if
                                    !after_decimal.is_ascii_digit() &&
                                    after_decimal != '_' &&
                                    after_decimal != '.'
                                {
                                    self.pos -= 1;
                                    break; // this is not a decimal part of the number, it's index into integer
                                } else {
                                    lexeme.push('.');
                                    kind = TokenKind::Float; // yeah ill have the regular please
                                    continue;
                                }
                            } else {
                                self.pos -= 1;
                                break; // 999. followed by EOF, stupid edge case garbage nonsense
                            }
                        }

                        self.advance();
                        lexeme.push(next_ch);
                    }
                    tokens.push(Token { kind, span: start..self.pos, lexeme });
                }
                _ =>
                    errors.push(
                        Error::new(
                            ErrorKind::IllegalCharacter,
                            start..self.pos,
                            "this character is not allowed",
                            true
                        )
                    ),
            }
            self.advance();
        }

        // sneak a little EOF to cap off the token stream
        tokens.push(Token::eof(self.pos..self.pos));
        return (tokens, errors);
    }
}

impl<'a> Lexer<'a> {
    /// Returns the character at current position <=> `pos` is not the end
    fn current(&self) -> Option<char> {
        if self.source.len() > self.pos {
            return Some(self.source.as_bytes()[self.pos] as char);
        }
        return None;
    }

    /// Returns the character 1 position ahead of the current position <=> `pos+1` is not the end
    fn peek(&self) -> Option<char> {
        if self.source.len() > self.pos + 1 {
            return Some(self.source.as_bytes()[self.pos + 1] as char);
        }
        return None;
    }

    /// Peeks ahead one and returns whether or not the next character equals
    /// the one provided `next_ch`. Also advances if the expected char was found.
    fn expect(&mut self, next_ch: char) -> bool {
        if let Some(ch) = self.peek() {
            if ch == next_ch {
                self.advance();
                return true;
            }
        }
        return false;
    }

    /// Moves the position of the lexer ahead by one <=> it isn't at the end of the stream
    fn advance(&mut self) {
        if self.source.len() > self.pos {
            self.pos += 1;
        }
    }
}
