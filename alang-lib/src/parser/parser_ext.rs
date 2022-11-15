use crate::{
    errors::*,
    token::{Token, TokenType},
};

use super::Parser;

pub trait ParserExt {
    fn current_line(&self) -> usize;
    fn current_column(&self) -> usize;

    fn peek(&self) -> &Token;
    fn eat(&mut self) -> Result<Token, Error>;
    fn expect(&mut self, expected: TokenType) -> Result<Token, Error>;
    fn is_eof(&self) -> bool;
}

impl ParserExt for Parser {
    fn current_line(&self) -> usize {
        self.peek().line
    }

    fn current_column(&self) -> usize {
        self.peek().column
    }

    fn peek(&self) -> &Token {
        &self.tokens[0]
    }

    fn eat(&mut self) -> Result<Token, Error> {
        let token = match self.tokens.pop_front() {
            Some(token) => token,
            None => return Err(UnexpectedEOF::new(0, 0).into()),
        };
        Ok(token)
    }

    fn expect(&mut self, expected: TokenType) -> Result<Token, Error> {
        let token = self.eat()?;
        println!("{:?}", token);

        if token.token_type == expected || expected == TokenType::EOF {
            Ok(token)
        } else {
            Err(
                UnexpectedToken::new(token.token_type, Some(expected), token.line, token.column)
                    .into(),
            )
        }
    }

    fn is_eof(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }
}
