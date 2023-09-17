use anyhow::{anyhow, Error};

#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn scan_token(&mut self) -> Result<Option<Token>, Error> {
        match self.consume() {
            Some('(') => Ok(Some(self.create_token(TokenType::LeftParen))),
            Some(')') => Ok(Some(self.create_token(TokenType::RightParen))),
            Some('{') => Ok(Some(self.create_token(TokenType::LeftBrace))),
            Some('}') => Ok(Some(self.create_token(TokenType::RightBrace))),
            Some(',') => Ok(Some(self.create_token(TokenType::Comma))),
            Some('.') => Ok(Some(self.create_token(TokenType::Dot))),
            Some('-') => Ok(Some(self.create_token(TokenType::Minus))),
            Some('+') => Ok(Some(self.create_token(TokenType::Plus))),
            Some(';') => Ok(Some(self.create_token(TokenType::Semicolon))),
            Some('*') => Ok(Some(self.create_token(TokenType::Star))),
            Some('\n') => Ok(None),
            Some(' ') => Ok(None),
            Some('\t') => Ok(None),
            Some('!') => Ok(Some(self.peek_and_decide(
                '=',
                TokenType::BangEqual,
                TokenType::Bang,
            ))),
            Some('=') => Ok(Some(self.peek_and_decide(
                '=',
                TokenType::EqualEqual,
                TokenType::Equal,
            ))),
            Some('<') => Ok(Some(self.peek_and_decide(
                '=',
                TokenType::LessEqual,
                TokenType::Less,
            ))),
            Some('>') => Ok(Some(self.peek_and_decide(
                '=',
                TokenType::GreaterEqual,
                TokenType::Greater,
            ))),
            Some('/') => {
                if self.consume_comment() {
                    Ok(None)
                } else {
                    Ok(Some(self.create_token(TokenType::Slash)))
                }
            }
            Some('"') => self.consume_string().map(|t| Some(t)),
            Some(c) if c.is_digit(10) => self.consume_number().map(|t| Some(t)),
            Some(c) => Err(anyhow!(format!(
                "unrecognized character on line {}: {}",
                self.line, c
            ))),
            None => Ok(None),
        }
    }

    fn create_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            lexeme: self.source[self.start..self.current].to_string(),
            line: self.line,
        }
    }

    fn consume(&mut self) -> Option<char> {
        let result = self.source.chars().nth(self.current);
        self.current += 1;
        result
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn peek_and_decide(
        &mut self,
        candidate: char,
        token_if_match: TokenType,
        token_if_no_match: TokenType,
    ) -> Token {
        match self.peek() {
            Some(c) if c == candidate => {
                self.consume();
                self.create_token(token_if_match)
            }
            Some(_) | None => self.create_token(token_if_no_match),
        }
    }

    fn consume_comment(&mut self) -> bool {
        match self.peek() {
            Some('/') => {
                loop {
                    match self.consume() {
                        Some('\n') => break,
                        Some(_) => (),
                        None => break,
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn consume_string(&mut self) -> Result<Token, Error> {
        loop {
            match self.consume() {
                Some('\n') => self.line += 1,
                Some('"') => break,
                Some(_) => (),
                None => {
                    return Err(anyhow!(format!(
                        "unterminated string on line {}",
                        self.line
                    )))
                }
            }
        }
        Ok(Token {
            token_type: TokenType::String,
            lexeme: self.source[self.start + 1..self.current - 1].to_string(),
            line: self.line,
        })
    }

    fn consume_number(&mut self) -> Result<Token, Error> {
        loop {
            match self.consume() {
                Some(c) if c.is_digit(10) => (),
                Some(c) if c == '.' => match self.peek_next() {
                    Some(next) if next.is_digit(10) => loop {
                        match self.consume() {
                            Some(c) if c.is_digit(10) => (),
                            Some(_) | None => {
                                return Ok(Token {
                                    token_type: TokenType::Number,
                                    lexeme: self.source[self.start..self.current - 1].to_string(),
                                    line: self.line,
                                })
                            }
                        }
                    },
                    Some(_) | None => {
                        return Err(anyhow!(format!("invalid number on line {}", self.line)))
                    }
                },
                Some(_) | None => break,
            }
        }
        Ok(Token {
            token_type: TokenType::Number,
            lexeme: self.source[self.start..self.current - 1].to_string(),
            line: self.line,
        })
    }
}

pub fn scan_tokens(source: String) -> Vec<Token> {
    let source_length = source.len();
    let mut result = Vec::new();
    let mut scanner = Scanner {
        start: 1,
        current: 0,
        line: 1,
        source,
    };

    while scanner.current < source_length {
        scanner.start = scanner.current;
        let token = scanner.scan_token();
        match token {
            Ok(Some(t)) => result.push(t),
            Ok(None) => (),
            Err(err) => println!("{}", err),
        }
    }
    result
}
