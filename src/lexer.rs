use crate::{ span::Span, token::{ Token, TokenKind } };

pub struct Lexer<'a> {
    pub tokens: Vec<Token>,
    source: &'a String,
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Initializes a new lexer with the given source
    pub fn new(source: &'a String) -> Lexer<'a> {
        Lexer {
            source,
            tokens: vec![],
            pos: 0,
        }
    }

    pub fn lex(&mut self) {
        while let Some(ch) = self.current() {
            let start = self.pos;

            match ch {
                // whitespace ignore
                ' ' | '\t' | '\r' | '\n' => {}

                // grouping operators
                '(' => self.push(TokenKind::LParen, start..start, "("),
                ')' => self.push(TokenKind::RParen, start..start, ")"),
                '[' => self.push(TokenKind::LBrac, start..start, "["),
                ']' => self.push(TokenKind::RBrac, start..start, "]"),
                '{' => self.push(TokenKind::LCurl, start..start, "{"),
                '}' => self.push(TokenKind::RCurl, start..start, "}"),

                '=' => self.push(TokenKind::Equal, start..start, "="),

                // other operators/symbols
                '-' => if self.expect('>') {
                    self.push(TokenKind::RArrow, start..self.pos, "->");
                } else {
                    self.push(TokenKind::Minus, start..self.pos, "-");
                }

                ':' => self.push(TokenKind::Colon, start..start, ":"),
                ';' => self.push(TokenKind::Semicolon, start..start, ";"),
                ',' => self.push(TokenKind::Comma, start..start, ","),
                '.' => self.push(TokenKind::Dot, start..start, "."),

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
                            None => panic!("Unterminated string literal, yo"),
                        }
                    }

                    self.tokens.push(Token {
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
                    self.tokens.push(Token { kind, span: start..self.pos, lexeme });
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
                    self.tokens.push(Token { kind, span: start..self.pos, lexeme });
                }
                _ => panic!("Unexpected token: '{}'", ch),
            }
            self.advance();
        }
        self.tokens.push(Token::eof(self.pos..self.pos));
    }

    /// Just prints out the entirety of the tokens vector
    pub fn dump(&self) {
        println!("{:#?}", self.tokens);
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

    /// Takes some basic token paramteres and pushes a new token to the output.
    /// Takes lexeme as as string slice and turns it into an owned string upon token instantiation.
    fn push(&mut self, kind: TokenKind, span: Span, lexeme: &str) {
        self.tokens.push(Token { kind, span, lexeme: lexeme.to_string() });
    }
}
