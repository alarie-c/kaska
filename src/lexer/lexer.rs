use crate::{ common::errors::{ Error, ErrorBuffer, ErrorKind, ErrorWriter }, throw };
use super::token::{ Tk, Token };

pub struct Lexer<'a> {
    source: &'a String,
    errors: ErrorBuffer,
    pos: usize,
}

impl<'a> ErrorWriter for Lexer<'a> {
    fn error(&mut self, error: Error) {
        self.errors.push(error);
    }

    fn dump_errors(&mut self) -> ErrorBuffer {
        return self.errors.drain(0..).collect();
    }
}

// ----------------------------------------------------------------- \\
// TOKENIZER IMPLEMENTATION
// ----------------------------------------------------------------- \\

impl<'a> Lexer<'a> {
    /// Initializes a new lexer with the given source
    pub fn new(source: &'a String) -> Lexer<'a> {
        Lexer {
            source,
            errors: vec![],
            pos: 0,
        }
    }

    /// Takes the input given to the lexer and iterates through, creating tokens
    /// and eventually returning them as a vector
    pub fn lex(&mut self) -> (Vec<Token>, ErrorBuffer) {
        let mut tokens = Vec::<Token>::new();

        while let Some(ch) = self.current() {
            let start = self.pos;

            match ch {
                // whitespace ignore
                ' ' | '\t' | '\r' => {}
                '\n' => tokens.push(Token::new(Tk::Newline, start..start, "\\n")),

                // grouping operators
                '(' => tokens.push(Token::new(Tk::LParen, start..start, "(")),
                ')' => tokens.push(Token::new(Tk::RParen, start..start, ")")),
                '[' => tokens.push(Token::new(Tk::LBrac, start..start, "[")),
                ']' => tokens.push(Token::new(Tk::RBrac, start..start, "]")),
                '{' => tokens.push(Token::new(Tk::LCurl, start..start, "{")),
                '}' => tokens.push(Token::new(Tk::RCurl, start..start, "}")),

                // tokenize ellipsis
                '.' => if self.expect('.') {
                    if self.expect('.') {
                        tokens.push(Token::new(Tk::Ellipsis, start..self.pos, "..."));
                    } else {
                        self.pos -= 1; // go back one
                        tokens.push(Token::new(Tk::Dot, start..self.pos, "."));
                    }
                } else {
                    tokens.push(Token::new(Tk::Dot, start..self.pos, "."));
                }

                // double wide arithmetic operators
                '+' => if self.expect('+') {
                    tokens.push(Token::new(Tk::PlusPlus, start..self.pos, "++"));
                } else if self.expect('=') {
                    tokens.push(Token::new(Tk::PlusEqual, start..self.pos, "+="));
                } else {
                    tokens.push(Token::new(Tk::Plus, start..self.pos, "+"));
                }

                '-' => if self.expect('-') {
                    tokens.push(Token::new(Tk::MinusMinus, start..self.pos, "--"));
                } else if self.expect('=') {
                    tokens.push(Token::new(Tk::MinusEqual, start..self.pos, "-="));
                } else if self.expect('>') {
                    tokens.push(Token::new(Tk::RArrow, start..self.pos, "->"));
                } else {
                    tokens.push(Token::new(Tk::Minus, start..self.pos, "-"));
                }

                // triple wide arithmetic operators
                '*' => if self.expect('*') {
                    if self.expect('=') {
                        tokens.push(Token::new(Tk::StarStarEqual, start..self.pos, "**="));
                    } else {
                        tokens.push(Token::new(Tk::StarStar, start..self.pos, "**"));
                    }
                } else if self.expect('=') {
                    tokens.push(Token::new(Tk::StarEqual, start..self.pos, "*="));
                } else {
                    tokens.push(Token::new(Tk::Star, start..self.pos, "*"));
                }

                '/' => if self.expect('/') {
                    if self.expect('=') {
                        tokens.push(Token::new(Tk::SlashSlashEqual, start..self.pos, "//="));
                    } else {
                        tokens.push(Token::new(Tk::SlashSlash, start..self.pos, "//"));
                    }
                } else if self.expect('=') {
                    tokens.push(Token::new(Tk::SlashEqual, start..self.pos, "/="));
                } else {
                    tokens.push(Token::new(Tk::Slash, start..self.pos, "/"));
                }

                // comparison operators
                '<' => if self.expect('=') {
                    tokens.push(Token::new(Tk::LessEqual, start..self.pos, "<="));
                } else {
                    tokens.push(Token::new(Tk::Less, start..self.pos, "<"));
                }
                '>' => if self.expect('=') {
                    tokens.push(Token::new(Tk::MoreEqual, start..self.pos, ">="));
                } else {
                    tokens.push(Token::new(Tk::More, start..self.pos, ">"));
                }
                '=' => if self.expect('=') {
                    tokens.push(Token::new(Tk::EqualEqual, start..self.pos, "=="));
                } else {
                    tokens.push(Token::new(Tk::Equal, start..self.pos, "="));
                }
                '!' => if self.expect('=') {
                    tokens.push(Token::new(Tk::BangEqual, start..self.pos, "!="));
                } else {
                    tokens.push(Token::new(Tk::Bang, start..self.pos, "!"));
                }

                // logical operators
                '|' => if self.expect('|') {
                    tokens.push(Token::new(Tk::PipePipe, start..self.pos, "||"));
                } else {
                    tokens.push(Token::new(Tk::Pipe, start..self.pos, "|"));
                }
                '&' => if self.expect('&') {
                    tokens.push(Token::new(Tk::Amprsnd, start..self.pos, "&&"));
                } else {
                    tokens.push(Token::new(Tk::AmprsndAmprsnd, start..self.pos, "&"));
                }

                '%' => tokens.push(Token::new(Tk::Modulo, start..start, "%")),
                ':' => tokens.push(Token::new(Tk::Colon, start..start, ":")),
                ';' => tokens.push(Token::new(Tk::Semicolon, start..start, ";")),
                ',' => tokens.push(Token::new(Tk::Comma, start..start, ",")),
                '$' => tokens.push(Token::new(Tk::Sigil, start..start, "$")),

                '"' => {
                    let mut lexeme = String::new();

                    // consume while valid identifier
                    'string: loop {
                        match self.peek() {
                            // if a valid cahracter comes next
                            Some(next_ch) => if next_ch == '\\' {
                                unimplemented!("Escape sequences not support yet brochacho... </3");
                            } else if next_ch == '"' {
                                self.advance();
                                break 'string;
                            } else {
                                self.advance();
                                lexeme.push(next_ch);
                            }

                            // uh oh cherio
                            None => {
                                self.error(
                                    throw!(
                                        SyntaxError,
                                        start..self.pos,
                                        "string literal is missing a closing '\"'"
                                    )
                                );
                                break;
                            }
                        }
                    }

                    tokens.push(Token {
                        kind: Tk::String,
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

                    let kind = Tk::from_lexeme(&lexeme);
                    tokens.push(Token { kind, span: start..self.pos, lexeme });
                }

                '0'..='9' => {
                    let mut lexeme = String::from(ch);
                    let mut kind = Tk::Integer;

                    // consume while valid number
                    while let Some(next_ch) = self.peek() {
                        if !next_ch.is_ascii_digit() && next_ch != '_' && next_ch != '.' {
                            break;
                        }

                        // update token kind when first decimal encountered
                        if next_ch == '.' {
                            if kind == Tk::Float {
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
                                    kind = Tk::Float; // yeah ill have the regular please
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
                '#' => {
                    while let Some(next_ch) = self.peek() {
                        self.advance();
                        if next_ch == '\n' {
                            break;
                        }
                    }
                }
                _ =>
                    self.error(
                        throw!(IllegalCharacter, start..self.pos, "this character is not allowed")
                    ),
            }
            self.advance();
        }

        // sneak a little EOF to cap off the token stream
        tokens.push(Token::eof(self.pos..self.pos));
        return (tokens, self.dump_errors());
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
