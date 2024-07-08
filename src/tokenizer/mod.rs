use peekmore::{PeekMore, PeekMoreIterator};
use std::{fmt::Display, str::Chars};

struct Keyword {
    name: &'static str,
    token: Token,
}

const RESERVED_KEYWORDS: [Keyword; 16] = [
    Keyword {
        name: "var",
        token: Token::Var,
    },
    Keyword {
        name: "and",
        token: Token::And,
    },
    Keyword {
        name: "class",
        token: Token::Class,
    },
    Keyword {
        name: "else",
        token: Token::Else,
    },
    Keyword {
        name: "false",
        token: Token::False,
    },
    Keyword {
        name: "for",
        token: Token::For,
    },
    Keyword {
        name: "fun",
        token: Token::Fun,
    },
    Keyword {
        name: "if",
        token: Token::If,
    },
    Keyword {
        name: "nil",
        token: Token::Nil,
    },
    Keyword {
        name: "or",
        token: Token::Or,
    },
    Keyword {
        name: "print",
        token: Token::Print,
    },
    Keyword {
        name: "return",
        token: Token::Return,
    },
    Keyword {
        name: "super",
        token: Token::Super,
    },
    Keyword {
        name: "this",
        token: Token::This,
    },
    Keyword {
        name: "true",
        token: Token::True,
    },
    Keyword {
        name: "while",
        token: Token::While,
    },
];

#[derive(Clone)]
pub struct TokenizerError {
    pub line: usize,
    pub message: String,
}

#[derive(Clone)]
pub enum Token {
    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
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

    // Single-character tokens
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
    LessEqual,
    Greater,
    GreaterEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(String),
    Invalid(TokenizerError),

    // Whitespace
    WhiteSpace,

    // End of file
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::WhiteSpace => write!(f, ""),
            Token::And => write!(f, "AND and null"),
            Token::Class => write!(f, "CLASS class null"),
            Token::Else => write!(f, "ELSE else null"),
            Token::False => write!(f, "FALSE false null"),
            Token::For => write!(f, "FOR for null"),
            Token::Fun => write!(f, "FUN fun null"),
            Token::If => write!(f, "IF if null"),
            Token::Nil => write!(f, "NIL nil null"),
            Token::Or => write!(f, "OR or null"),
            Token::Print => write!(f, "PRINT print null"),
            Token::Return => write!(f, "RETURN return null"),
            Token::Super => write!(f, "SUPER super null"),
            Token::This => write!(f, "THIS this null"),
            Token::True => write!(f, "TRUE true null"),
            Token::While => write!(f, "WHILE while null"),
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
            Token::String(s) => write!(f, "STRING \"{}\" {}", s, s),
            Token::Number(s) => {
                let f64_str = match s.parse::<f64>() {
                    Ok(f) => format!("{:?}", f),
                    Err(_) => s.clone(),
                };
                write!(f, "NUMBER {} {}", s, f64_str)
            }
            Token::Invalid(s) => write!(f, "[line {}] Error: {}", s.line, s.message),
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekmore();
    let mut line = 1;

    while let Some(c) = chars.next() {
        let token = match c {
            ' ' | '\t' => Token::WhiteSpace,
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
                        while let Some(c) = chars.next() {
                            if c == '\n' {
                                line += 1;
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
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = tokenize_identifier(c, &mut chars);
                identifier
            }
            '\n' => {
                line += 1;
                Token::WhiteSpace
            }
            _ => Token::Invalid(TokenizerError {
                line,
                message: format!("Unexpected character: {}", c),
            }),
        };

        tokens.push(token);
    }

    tokens.push(Token::EOF);
    tokens
}

fn tokenize_number(first_char: char, chars: &mut PeekMoreIterator<Chars>) -> Token {
    let mut number = String::new();
    let mut decimal = false;
    number.push(first_char);
    while let Some(&c) = chars.peek() {
        if c.is_digit(10) {
            number.push(c);
            chars.next();
        } else if c == '.' {
            if decimal {
                break;
            }
            let next_2c = chars.peek_nth(2);
            if next_2c.is_none() || !next_2c.unwrap().is_digit(10) {
                break;
            }
            decimal = true;
            number.push(c);
            chars.next();
        } else {
            break;
        }
    }
    if &number[number.len() - 1..] == "." {
        number.push('0');
    }
    Token::Number(number)
}

fn tokenize_string(chars: &mut PeekMoreIterator<Chars>) -> Token {
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
            message: "Unterminated string.".to_string(),
        });
    }
    Token::String(string)
}

fn tokenize_identifier(first_char: char, chars: &mut PeekMoreIterator<Chars>) -> Token {
    let mut identifier = String::new();
    identifier.push(first_char);
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
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
