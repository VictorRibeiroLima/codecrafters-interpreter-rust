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
pub struct TokenizerError {
    pub line: usize,
    pub message: String,
}

#[derive(Clone)]
pub enum Token {
    Var,
    Equal,
    EqualEqual,
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
    Bang,
    BangEqual,
    Less,
    WhiteSpace,
    LessEqual,
    Greater,
    GreaterEqual,
    Identifier(String),
    String(String),
    Number(String),
    Invalid(TokenizerError),
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::WhiteSpace => write!(f, ""),
            Token::Var => write!(f, "VAR var null"),
            Token::Equal => write!(f, "EQUAL = null"),
            Token::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
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
            Token::Bang => write!(f, "BANG ! null"),
            Token::BangEqual => write!(f, "BANG_EQUAL != null"),
            Token::Less => write!(f, "LESS < null"),
            Token::LessEqual => write!(f, "LESS_EQUAL <= null"),
            Token::Greater => write!(f, "GREATER > null"),
            Token::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            Token::Identifier(s) => write!(f, "IDENTIFIER {} null", s),
            Token::String(s) => write!(f, "STRING {} \"{}\"", s, s),
            Token::Number(s) => write!(f, "NUMBER {} {}", s, s),
            Token::Invalid(s) => write!(f, "[line {}] Error: {}", s.line, s.message),
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut pushing = true;
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut line = 1;

    while let Some(c) = chars.next() {
        let token = match c {
            ' ' => Token::WhiteSpace,
            '=' => {
                if let Some(&next_char) = chars.peek() {
                    if next_char == '=' {
                        chars.next();
                        Token::EqualEqual
                    } else {
                        Token::Equal
                    }
                } else {
                    Token::Equal
                }
            }
            ';' => Token::Semicolon,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '*' => Token::Star,
            '.' => Token::Dot,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => {
                if let Some(&next_char) = chars.peek() {
                    if next_char == '/' {
                        pushing = false;
                        while let Some(c) = chars.next() {
                            if c == '\n' {
                                pushing = true;
                                break;
                            }
                        }
                        Token::WhiteSpace
                    } else {
                        Token::Slash
                    }
                } else {
                    Token::Slash
                }
            }
            '!' => {
                if let Some(&next_char) = chars.peek() {
                    if next_char == '=' {
                        chars.next();
                        Token::BangEqual
                    } else {
                        Token::Bang
                    }
                } else {
                    Token::Bang
                }
            }
            '<' => {
                if let Some(&next_char) = chars.peek() {
                    if next_char == '=' {
                        chars.next();
                        Token::LessEqual
                    } else {
                        Token::Less
                    }
                } else {
                    Token::Less
                }
            }
            '>' => {
                if let Some(&next_char) = chars.peek() {
                    if next_char == '=' {
                        chars.next();
                        Token::GreaterEqual
                    } else {
                        Token::Greater
                    }
                } else {
                    Token::Greater
                }
            }
            '0'..='9' => {
                let number = tokenize_number(c, &mut chars);
                number
            }
            '"' => {
                let string = tokenize_string(&mut chars);
                string
            }
            'a'..='z' | 'A'..='Z' => {
                let identifier = tokenize_identifier(c, &mut chars);
                identifier
            }
            '\n' => {
                line += 1;
                pushing = true;
                Token::WhiteSpace
            }
            _ => Token::Invalid(TokenizerError {
                line,
                message: format!("Unexpected character: {}", c),
            }),
        };

        if pushing {
            tokens.push(token);
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
            return Token::Invalid(TokenizerError {
                line: 1,
                message: format!("Unexpected character: {}", c),
            });
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
        return Token::Invalid(TokenizerError {
            line: 1,
            message: "Unterminated string".to_string(),
        });
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
