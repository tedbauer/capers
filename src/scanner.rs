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
    start: usize,
    current: usize,
    line: usize,
}

fn create_token(scanner: &Scanner, source: &str, token_type: TokenType) -> Token {
    Token {
        token_type,
        lexeme: source[scanner.start..scanner.current+1].to_string(),
        line: scanner.line,
    }
}

fn scan_token(source: &str, scanner: &mut Scanner) -> Option<Token> {
    let result = match source.chars().nth(scanner.current) {
        Some('(') => Some(create_token(scanner, source, TokenType::LeftParen)),
        Some(')') => Some(create_token(scanner, source, TokenType::RightParen)),
        Some('{') => Some(create_token(scanner, source, TokenType::LeftBrace)),
        Some('}') => Some(create_token(scanner, source, TokenType::RightBrace)),
        Some(',') => Some(create_token(scanner, source, TokenType::Comma)),
        Some('.') => Some(create_token(scanner, source, TokenType::Dot)),
        Some('-') => Some(create_token(scanner, source, TokenType::Minus)),
        Some('+') => Some(create_token(scanner, source, TokenType::Plus)),
        Some(';') => Some(create_token(scanner, source, TokenType::Semicolon)),
        Some('*') => Some(create_token(scanner, source, TokenType::Star)),
        Some(c) => None,
        None => None
    };
    scanner.current += 1;
    result
}

pub fn scan_tokens(source: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut scanner = Scanner {
        start: 1,
        current: 0,
        line: 1,
    };

    while scanner.current < source.len() {
        scanner.start = scanner.current;
        let token = scan_token(source, &mut scanner);
        if let Some(t) = token {
            result.push(t);
        }
    }
    result
}
