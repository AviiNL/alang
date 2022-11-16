// https://en.wikipedia.org/wiki/Order_of_operations#Programming_languages

pub(crate) mod ast;

mod parser_ext;
use std::collections::VecDeque;

use parser_ext::*;

use crate::{
    errors::*,
    lexer::tokenize,
    token::{Token, TokenType},
};

use self::ast::ExpressionType;

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn produce_ast(input: &str) -> Result<ast::Program, Error> {
        let tokens = tokenize(input)?;
        let mut parser = Parser { tokens };

        let body = parser.parse_block(None)?;

        Ok(ast::Program { body })
    }

    fn is_end_token(&self, tokens: Option<&[TokenType]>) -> bool {
        if let Some(tokens) = tokens {
            tokens.contains(&self.peek().token_type)
        } else {
            false
        }
    }

    fn parse_block(
        &mut self,
        end_token: Option<&[TokenType]>,
    ) -> Result<Vec<ast::Expression>, Error> {
        let mut body = Vec::new();

        while !self.is_eof() && !self.is_end_token(end_token) {
            let expr = self.parse()?;
            body.push(expr);
        }

        Ok(body)
    }

    fn parse(&mut self) -> Result<ast::Expression, Error> {
        let expr = self.parse_expression();

        while self.peek().token_type == TokenType::EOL {
            self.eat()?;
        }

        expr
    }

    fn parse_expression(&mut self) -> Result<ast::Expression, Error> {
        self.parse_assignment()
    }

    // Assignment operators (right to left)
    // Todo: +=   -=   *=   /=   %=   &=   |=   ^=   <<=   >>=
    fn parse_assignment(&mut self) -> Result<ast::Expression, Error> {
        let mut left = self.parse_function()?;

        while self.peek().token_type == TokenType::Equal {
            self.eat()?;
            let right = self.parse_function()?;

            let line = left.line;
            let column = left.column;

            left = ast::Expression::new(
                ExpressionType::Assignment(ast::Assignment {
                    left: Box::new(left),
                    right: Box::new(right),
                }),
                line,
                column,
            );
        }

        Ok(left)
    }

    // Function declaration
    fn parse_function(&mut self) -> Result<ast::Expression, Error> {
        if self.peek().token_type != TokenType::Function {
            return self.parse_or();
        }

        self.expect(TokenType::Function)?;

        let name = self.parse_identifier()?;

        self.expect(TokenType::LeftParen)?;

        let mut parameters = Vec::new();

        while self.peek().token_type != TokenType::RightParen {
            let parameter = self.parse_identifier()?;

            parameters.push(parameter);

            if self.peek().token_type == TokenType::Comma {
                self.eat()?;
            }
        }

        self.expect(TokenType::RightParen)?;

        self.expect(TokenType::EOL)?;

        let body = self.parse_block(Some(&[TokenType::End]))?;

        self.expect(TokenType::End)?;
        self.expect(TokenType::EOL)?;

        let line = name.line;
        let column = name.column;

        Ok(ast::Expression::new(
            ExpressionType::Function(ast::Function {
                name: Box::new(name),
                parameters,
                body,
            }),
            line,
            column,
        ))
    }

    // Conditional expression (ternary)

    // Logical OR
    fn parse_or(&mut self) -> Result<ast::Expression, Error> {
        let mut left = self.parse_and()?;

        while self.peek().token_type == TokenType::Or {
            let operator = self.eat()?.into();
            let right = self.parse_and()?;

            let line = left.line;
            let column = left.column;

            left = ast::Expression::new(
                ast::ExpressionType::Binary(ast::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }),
                line,
                column,
            );
        }

        Ok(left)
    }

    // Logical AND
    fn parse_and(&mut self) -> Result<ast::Expression, Error> {
        let mut left = self.parse_equality()?;

        while self.peek().token_type == TokenType::And {
            let operator = self.eat()?.into();
            let right = self.parse_equality()?;

            let line = left.line;
            let column = left.column;

            left = ast::Expression::new(
                ast::ExpressionType::Binary(ast::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }),
                line,
                column,
            );
        }

        Ok(left)
    }

    // Bitwise inclusive (normal) OR

    // Bitwise exclusive OR (XOR)

    // Bitwise AND

    // Comparisons: equal and not equal
    fn parse_equality(&mut self) -> Result<ast::Expression, Error> {
        let mut left = self.parse_comparison()?;

        while self.peek().token_type == TokenType::EqualEqual
            || self.peek().token_type == TokenType::BangEqual
            || self.peek().token_type == TokenType::Is
        // Type check
        {
            let operator = self.eat()?.into();
            let right = self.parse_comparison()?;

            let line = left.line;
            let column = left.column;

            left = ast::Expression::new(
                ast::ExpressionType::Binary(ast::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }),
                line,
                column,
            );
        }

        Ok(left)
    }

    // Comparisons: less-than and greater-than
    fn parse_comparison(&mut self) -> Result<ast::Expression, Error> {
        let mut left = self.parse_term()?;

        while self.peek().token_type == TokenType::Greater
            || self.peek().token_type == TokenType::GreaterEqual
            || self.peek().token_type == TokenType::Less
            || self.peek().token_type == TokenType::LessEqual
        {
            let operator = self.eat()?.into();
            let right = self.parse_term()?;

            let line = left.line;
            let column = left.column;

            left = ast::Expression::new(
                ast::ExpressionType::Binary(ast::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }),
                line,
                column,
            );
        }

        Ok(left)
    }

    // Bitwise shift left and right

    // Addition and subtraction
    fn parse_term(&mut self) -> Result<ast::Expression, Error> {
        let mut left = self.parse_factor()?;

        while self.peek().token_type == TokenType::Plus
            || self.peek().token_type == TokenType::Minus
        {
            let operator = self.eat()?.into();
            let right = self.parse_factor()?;

            let line = left.line;
            let column = left.column;

            left = ast::Expression::new(
                ast::ExpressionType::Binary(ast::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }),
                line,
                column,
            );
        }

        Ok(left)
    }

    // Multiplication, division, modulo
    fn parse_factor(&mut self) -> Result<ast::Expression, Error> {
        let mut left = self.parse_exponent()?;

        while self.peek().token_type == TokenType::Star
            || self.peek().token_type == TokenType::Slash
            || self.peek().token_type == TokenType::Percent
        {
            let operator = self.eat()?.into();
            let right = self.parse_exponent()?;

            let line = left.line;
            let column = left.column;

            left = ast::Expression::new(
                ast::ExpressionType::Binary(ast::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }),
                line,
                column,
            );
        }

        Ok(left)
    }

    // Exponentiation
    fn parse_exponent(&mut self) -> Result<ast::Expression, Error> {
        let mut left = self.parse_unary()?;

        while self.peek().token_type == TokenType::Caret {
            let operator = self.eat()?.into();
            let right = self.parse_unary()?;

            let line = left.line;
            let column = left.column;

            left = ast::Expression::new(
                ast::ExpressionType::Binary(ast::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }),
                line,
                column,
            );
        }

        Ok(left)
    }

    // !   ~   -   +   *   &   sizeof   type cast   ++   --
    fn parse_unary(&mut self) -> Result<ast::Expression, Error> {
        if self.peek().token_type == TokenType::Bang
            || self.peek().token_type == TokenType::Minus
            || self.peek().token_type == TokenType::Plus
        {
            let operator = self.eat()?.into();
            let right = self.parse_unary()?;

            let line = right.line;
            let column = right.column;

            Ok(ast::Expression::new(
                ExpressionType::Unary(ast::Unary {
                    operator,
                    right: Box::new(right),
                }),
                line,
                column,
            ))
        } else {
            self.parse_function_call()
        }
    }

    // Function call
    fn parse_function_call(&mut self) -> Result<ast::Expression, Error> {
        let mut left = self.parse_primary()?;

        if self.peek().token_type == TokenType::LeftParen {
            let parameters = self.parse_arguments()?;

            let line = left.line;
            let column = left.column;

            left = ast::Expression::new(
                ast::ExpressionType::Call(ast::Call {
                    name: Box::new(left),
                    parameters,
                }),
                line,
                column,
            );
        }

        Ok(left)
    }

    // Function Arguments
    fn parse_arguments(&mut self) -> Result<Vec<ast::Expression>, Error> {
        let mut arguments = Vec::new();

        self.expect(TokenType::LeftParen)?; // Eat the left parenthesis

        while self.peek().token_type != TokenType::RightParen {
            arguments.push(self.parse_expression()?);

            if self.peek().token_type == TokenType::Comma {
                self.eat()?;
            }
        }

        self.expect(TokenType::RightParen)?; // Eat the right parenthesis

        Ok(arguments)
    }

    // function call, scope, array/member access, etc
    fn parse_primary(&mut self) -> Result<ast::Expression, Error> {
        let token = self.eat()?;

        match token.token_type {
            TokenType::Number(value) => Ok(ast::Expression::new(
                ExpressionType::Number(value),
                token.line,
                token.column,
            )),
            TokenType::String(value) => Ok(ast::Expression::new(
                ExpressionType::String(value),
                token.line,
                token.column,
            )),
            TokenType::Character(value) => Ok(ast::Expression::new(
                ExpressionType::Character(value),
                token.line,
                token.column,
            )),
            TokenType::Boolean(value) => Ok(ast::Expression::new(
                ExpressionType::Boolean(value),
                token.line,
                token.column,
            )),
            TokenType::Identifier(value) => Ok(ast::Expression::new(
                ExpressionType::Identifier(value),
                token.line,
                token.column,
            )),
            TokenType::Function => Ok(ast::Expression::new(
                ExpressionType::Type("function".to_string()),
                token.line,
                token.column,
            )),
            TokenType::LeftParen => {
                let expr = self.parse_expression()?;
                self.expect(TokenType::RightParen)?;

                let line = expr.line;
                let column = expr.column;

                Ok(ast::Expression::new(
                    ExpressionType::Grouping(ast::Grouping {
                        expression: Box::new(expr),
                    }),
                    line,
                    column,
                ))
            }
            TokenType::If => {
                let condition = self.parse_expression()?;

                self.expect(TokenType::Then)?;

                while self.peek().token_type == TokenType::EOL {
                    self.eat()?; // eat any lingering EOLS
                }

                let then_branch = self.parse_block(Some(&[TokenType::Else, TokenType::End]))?;

                let else_branch = if self.peek().token_type == TokenType::Else {
                    Some(self.parse_else()?)
                } else {
                    self.expect(TokenType::End)?;
                    None
                };

                let line = condition.line;
                let column = condition.column;

                Ok(ast::Expression::new(
                    ExpressionType::If(ast::If {
                        condition: Box::new(condition),
                        body: then_branch,
                        else_body: else_branch,
                    }),
                    line,
                    column,
                ))
            }
            TokenType::Return => {
                let value = self.parse_expression()?;

                if self.tokens.len() > 1 {
                    while self.peek().token_type == TokenType::EOL {
                        self.eat()?; // eat any lingering EOLS
                    }
                }

                let line = value.line;
                let column = value.column;

                Ok(ast::Expression::new(
                    ExpressionType::Return(ast::Return {
                        value: Box::new(value),
                    }),
                    line,
                    column,
                ))
            }
            token_type => {
                Err(UnexpectedToken::new(token_type, None, token.line, token.column).into())
            }
        }
    }

    fn parse_else(&mut self) -> Result<Vec<ast::Expression>, Error> {
        self.eat()?; // eat the else

        while self.peek().token_type == TokenType::EOL {
            self.eat()?; // eat any lingering EOLS
        }

        if self.peek().token_type == TokenType::If {
            println!("We got another if.");
            return Ok(vec![self.parse()?]);
        }

        let else_branch = self.parse_block(Some(&[TokenType::End, TokenType::Else]))?;

        self.expect(TokenType::End)?;

        Ok(else_branch)
    }

    fn parse_identifier(&mut self) -> Result<ast::Expression, Error> {
        let identifier = self.parse_primary()?;

        if let ExpressionType::Identifier(_) = identifier.expression_type {
            Ok(identifier)
        } else {
            Err(UnexpectedExpression::new(
                identifier.expression_type,
                Some(ExpressionType::Identifier(String::new())),
                identifier.line,
                identifier.column,
            )
            .into())
        }
    }
}
