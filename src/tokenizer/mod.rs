use std::{fmt::Display, iter::Peekable, str::Chars};

struct Keyword_ {
    name: &'static str,
    token: Token,
}

const RESERVED_KEYWORDS: [Keyword_; 1] = [Keyword_ {
    name: "var",
    token: Token::Var,
}];

#[derive(Clone)]
pub enum Token {
    Var,
    Identifier(String),
    Equal,
    String(String),
    Number(String),
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Slash,
    Invalid(String),
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Var => write!(f, "VAR var null"),
            Token::Identifier(s) => write!(f, "IDENTIFIER {} null", s),
            Token::Equal => write!(f, "EQUAL = null"),
            Token::String(s) => write!(f, "STRING {} \"{}\"", s, s),
            Token::Number(s) => write!(f, "NUMBER {} {}", s, s),
            Token::Semicolon => write!(f, "SEMICOLON ; null"),
            Token::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Token::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Token::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Token::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Token::Star => write!(f, "STAR * null"),
            Token::Dot => write!(f, "DOT . null"),
            Token::Comma => write!(f, "COMMA , null"),
            Token::Plus => write!(f, "PLUS + null"),
            Token::Minus => write!(f, "MINUS - null"),
            Token::Slash => write!(f, "SLASH / null"),
            Token::EOF => write!(f, "EOF  null"),
            Token::Invalid(s) => write!(f, "INVALID {} null", s),
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ' ' => continue,
            '=' => tokens.push(Token::Equal),
            ';' => tokens.push(Token::Semicolon),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            '*' => tokens.push(Token::Star),
            '.' => tokens.push(Token::Dot),
            ',' => tokens.push(Token::Comma),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '/' => tokens.push(Token::Slash),
            '0'..='9' => {
                let number = tokenize_number(c, &mut chars);
                tokens.push(number);
            }
            '"' => {
                let string = tokenize_string(&mut chars);
                tokens.push(string);
            }
            'a'..='z' | 'A'..='Z' => {
                let identifier = tokenize_identifier(c, &mut chars);
                tokens.push(identifier);
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }

    tokens.push(Token::EOF);
    tokens
}

fn tokenize_number(first_char: char, chars: &mut Peekable<Chars>) -> Token {
    let mut number = String::new();
    number.push(first_char);
    while let Some(&c) = chars.peek() {
        if c.is_digit(10) {
            number.push(c);
            chars.next();
        } else if c == ' ' || c == ';' {
            break;
        } else {
            number.push(c);
            chars.next();
            return Token::Invalid(number);
        }
    }
    Token::Number(number)
}

fn tokenize_string(chars: &mut Peekable<Chars>) -> Token {
    let mut string = String::new();
    let mut last_char = '"';
    while let Some(c) = chars.next() {
        last_char = c;
        if c == '"' {
            break;
        }
        string.push(c);
    }
    if last_char != '"' {
        return Token::Invalid(string);
    }
    Token::String(string)
}

fn tokenize_identifier(first_char: char, chars: &mut Peekable<Chars>) -> Token {
    let mut identifier = String::new();
    identifier.push(first_char);
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() {
            identifier.push(c);
            chars.next();
        } else {
            break;
        }
    }
    for keyword in RESERVED_KEYWORDS.iter() {
        if keyword.name == identifier {
            return keyword.token.clone();
        }
    }
    Token::Identifier(identifier)
}
