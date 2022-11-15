use std::collections::VecDeque;

use crate::{
    errors::*,
    token::{Token, TokenType},
};

static KEYWORDS: &[(&str, TokenType)] = &[
    ("true", TokenType::Boolean(true)),
    ("false", TokenType::Boolean(false)),
];

pub fn tokenize(input: &str) -> Result<VecDeque<Token>, Error> {
    let mut tokens = VecDeque::new();
    let mut line = 1;
    let mut column = 0;
    let mut chars = input.chars().peekable();

    let mut group: usize = 0;

    while let Some(c) = chars.next() {
        column += 1;
        match c {
            // Newline
            '\n' => {
                line += 1;
                column = 0;

                // if the last added token is a backslash, remove it and continue
                if let Some(Token {
                    token_type: TokenType::Backslash,
                    ..
                }) = tokens.back()
                {
                    tokens.pop_back();
                    continue;
                }

                if group > 0 {
                    return Err(UnexpectedEOL::new(line, column).into());
                }

                tokens.push_back(Token::new(TokenType::EOL, line, column));
            }

            // Identifier
            'A'..='Z' | 'a'..='z' | '_' => {
                let cur_col = column;
                let mut identifier = String::new();
                identifier.push(c);
                while let Some(&('A'..='Z' | 'a'..='z' | '0'..='9' | '_')) = chars.peek() {
                    identifier.push(chars.next().unwrap());
                    column += 1;
                }

                // Check if it's a keyword
                let token_type = KEYWORDS
                    .iter()
                    .find(|(keyword, _)| *keyword == identifier)
                    .map(|(_, token_type)| token_type.clone())
                    .unwrap_or(TokenType::Identifier(identifier));

                tokens.push_back(Token::new(token_type, line, cur_col));
            }

            // String literal
            '"' => {
                group += 1;
                let cur_col = column;
                let mut string = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '\\' {
                        chars.next(); // Skip the backslash
                        column += 1;
                        match chars.next() {
                            Some('n') => string.push('\n'),
                            Some('t') => string.push('\t'),
                            Some('r') => string.push('\r'),
                            Some('0') => string.push('\0'),
                            Some('\'') => string.push('\''),
                            Some('\"') => string.push('\"'),
                            Some('\\') => string.push('\\'),
                            Some(c) => {
                                return Err(InvalidEscapeCharacter::new(c, line, column).into())
                            }
                            _ => return Err(InvalidEscapeCharacter::new('\0', line, column).into()),
                        }
                        continue;
                    }

                    if c == '"' {
                        chars.next();
                        column += 1;
                        group -= 1;
                        break;
                    }
                    string.push(c);
                    chars.next();
                    column += 1;
                }

                tokens.push_back(Token::new(TokenType::String(string), line, cur_col));
            }
            // Character literal
            '\'' => {
                group += 1;
                let cur_col = column;
                let mut string = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '\\' {
                        chars.next(); // Skip the backslash
                        column += 1;
                        match chars.next() {
                            Some('n') => string.push('\n'),
                            Some('t') => string.push('\t'),
                            Some('r') => string.push('\r'),
                            Some('0') => string.push('\0'),
                            Some('\'') => string.push('\''),
                            Some('\"') => string.push('\"'),
                            Some('\\') => string.push('\\'),
                            Some(c) => {
                                return Err(InvalidEscapeCharacter::new(c, line, column).into())
                            }
                            _ => return Err(InvalidEscapeCharacter::new('\0', line, column).into()),
                        }
                        continue;
                    }

                    if c == '\'' {
                        chars.next();
                        column += 1;
                        group -= 1;
                        break;
                    }
                    string.push(c);
                    chars.next();
                    column += 1;
                }

                // ensure string length is 1
                if string.len() != 1 {
                    return Err(InvalidCharacterLiteral::new(line, cur_col).into());
                }

                // grab the character from the string
                let character = string.chars().next().unwrap();

                tokens.push_back(Token::new(TokenType::Character(character), line, cur_col));
            }
            // Numbers
            '0'..='9' => {
                let cur_col = column;
                // Primary Number
                let mut number = String::new();
                number.push(c);
                while let Some(&('0'..='9')) = chars.peek() {
                    number.push(chars.next().unwrap());
                    column += 1;
                }

                // Floating points
                if let Some(&'.') = chars.peek() {
                    number.push(chars.next().unwrap());
                    column += 1;
                    while let Some(&('0'..='9')) = chars.peek() {
                        number.push(chars.next().unwrap());
                        column += 1;
                    }
                }

                tokens.push_back(Token::new(
                    TokenType::Number(number.parse().unwrap()),
                    line,
                    cur_col,
                ));
            }

            // Booleans
            // Booleans are keywords, dummy

            // Arithmetic operators
            '+' => tokens.push_back(Token::new(TokenType::Plus, line, column)),
            '-' => tokens.push_back(Token::new(TokenType::Minus, line, column)),
            '*' => tokens.push_back(Token::new(TokenType::Star, line, column)),
            '/' => {
                // Comments
                if let Some(&'/') = chars.peek() {
                    chars.next();
                    column += 1;
                    while let Some(&c) = chars.peek() {
                        if c == '\n' {
                            break;
                        }
                        chars.next();
                        column += 1;
                    }
                    continue;
                } else if let Some(&'*') = chars.peek() {
                    chars.next();
                    column += 1;
                    while let Some(&c) = chars.peek() {
                        if c == '*' {
                            chars.next();
                            column += 1;
                            if let Some(&'/') = chars.peek() {
                                chars.next();
                                column += 1;
                                break;
                            }
                        } else {
                            chars.next();
                            column += 1;
                        }
                    }
                    continue;
                }
                tokens.push_back(Token::new(TokenType::Slash, line, column))
            }
            '%' => tokens.push_back(Token::new(TokenType::Percent, line, column)),
            '^' => tokens.push_back(Token::new(TokenType::Caret, line, column)),

            // Bitwise operators
            // Ampersand is parsed in And (logical) operator
            // Pipe is parsed in Or (logical) operator
            '~' => tokens.push_back(Token::new(TokenType::Tilde, line, column)),
            // LeftShift is parsed in Less (comparsion) operator
            // RightShift is parsed in Greater (comparsion) operator

            // Comparison
            '=' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    column += 1;
                    tokens.push_back(Token::new(TokenType::EqualEqual, line, column));
                } else {
                    tokens.push_back(Token::new(TokenType::Equal, line, column));
                }
            }
            '!' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    column += 1;
                    tokens.push_back(Token::new(TokenType::BangEqual, line, column));
                } else {
                    tokens.push_back(Token::new(TokenType::Bang, line, column));
                }
            }
            '<' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    column += 1;
                    tokens.push_back(Token::new(TokenType::LessEqual, line, column));
                } else if let Some(&'<') = chars.peek() {
                    chars.next();
                    column += 1;
                    tokens.push_back(Token::new(TokenType::LeftShift, line, column));
                } else {
                    tokens.push_back(Token::new(TokenType::Less, line, column));
                }
            }
            '>' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    column += 1;
                    tokens.push_back(Token::new(TokenType::GreaterEqual, line, column));
                } else if let Some(&'>') = chars.peek() {
                    chars.next();
                    column += 1;
                    tokens.push_back(Token::new(TokenType::RightShift, line, column));
                } else {
                    tokens.push_back(Token::new(TokenType::Greater, line, column));
                }
            }

            // Logical
            '&' => {
                if let Some(&'&') = chars.peek() {
                    chars.next();
                    column += 1;
                    tokens.push_back(Token::new(TokenType::And, line, column));
                } else {
                    tokens.push_back(Token::new(TokenType::Ampersand, line, column));
                }
            }
            '|' => {
                if let Some(&'|') = chars.peek() {
                    chars.next();
                    column += 1;
                    tokens.push_back(Token::new(TokenType::Or, line, column));
                } else {
                    tokens.push_back(Token::new(TokenType::Pipe, line, column));
                }
            }

            // Assingment
            // Equal is parsed in Comparison operator

            // Delimiters
            ',' => tokens.push_back(Token::new(TokenType::Comma, line, column)),
            ';' => tokens.push_back(Token::new(TokenType::Semicolon, line, column)),
            ':' => tokens.push_back(Token::new(TokenType::Colon, line, column)),
            '.' => tokens.push_back(Token::new(TokenType::Dot, line, column)),
            '(' => tokens.push_back(Token::new(TokenType::LeftParen, line, column)),
            ')' => tokens.push_back(Token::new(TokenType::RightParen, line, column)),
            '{' => tokens.push_back(Token::new(TokenType::LeftBrace, line, column)),
            '}' => tokens.push_back(Token::new(TokenType::RightBrace, line, column)),
            '[' => tokens.push_back(Token::new(TokenType::LeftBracket, line, column)),
            ']' => tokens.push_back(Token::new(TokenType::RightBracket, line, column)),
            '\\' => tokens.push_back(Token::new(TokenType::Backslash, line, column)),

            // Whitespace and unhandled characters
            _ => {
                if c.is_whitespace() {
                    continue;
                }

                return Err(UnhandledCharacter::new(c, line, column).into());
            }
        }
    }

    if group > 0 {
        return Err(UnexpectedEOF::new(line, column).into());
    }

    tokens.push_back(Token {
        token_type: TokenType::EOF,
        line: line + 1,
        column: 1,
    });

    Ok(tokens)
}
