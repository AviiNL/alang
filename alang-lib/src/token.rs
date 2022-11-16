#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals.
    Identifier(String),
    String(String),
    Character(char),
    Number(f64),
    Boolean(bool),

    // Keywords
    If,
    Else,
    End,
    Function,
    Return,
    Is,

    // Arithmatic Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,

    // Bitwise Operators
    Ampersand,
    Pipe,
    Tilde,
    LeftShift,
    RightShift,

    // Comparison
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical
    And,
    Or,
    Bang,

    // Assignment
    Equal,

    // Delimiters
    Backslash,
    Comma,
    Semicolon,
    Colon,
    Dot,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // End of Line
    EOL,

    // End of File
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            token_type,
            line,
            column,
        }
    }
}
