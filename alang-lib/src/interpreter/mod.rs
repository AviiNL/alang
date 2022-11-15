pub mod environment;

use crate::{
    errors::*,
    parser::ast,
    types::{
        boolean::BooleanVal, character::CharacterVal, number::NumberVal, string::StringVal,
        Arithmatic, BinaryOperation, BinaryOperationError, Logical, Operator, RuntimeType,
        RuntimeValue,
    },
};
pub use environment::Environment;

pub fn run(ast: &ast::Program, env: &mut Environment) -> Result<RuntimeValue, Error> {
    let mut last_value = RuntimeValue::Null;

    for expression in &ast.body {
        last_value = evaluate_expression(expression, env)?.value;
    }

    Ok(last_value)
}

fn evaluate_expression(
    expression: &ast::Expression,
    env: &mut Environment,
) -> Result<RuntimeType, Error> {
    let expr_type = &expression.expression_type;

    match expr_type {
        ast::ExpressionType::Identifier(value) => {
            let value = env.get(&value).ok_or_else(|| {
                UndefinedVariable::new(value.clone(), expression.line, expression.column).into()
            })?;

            Ok(RuntimeType {
                value,
                line: expression.line,
                column: expression.column,
            })
        }
        ast::ExpressionType::Number(value) => Ok(RuntimeType {
            value: RuntimeValue::Number(NumberVal::from(*value)),
            line: expression.line,
            column: expression.column,
        }),
        ast::ExpressionType::String(value) => Ok(RuntimeType {
            value: RuntimeValue::String(StringVal::from(value.clone())),
            line: expression.line,
            column: expression.column,
        }),
        ast::ExpressionType::Character(value) => Ok(RuntimeType {
            value: RuntimeValue::Character(CharacterVal::from(*value)),
            line: expression.line,
            column: expression.column,
        }),
        ast::ExpressionType::Boolean(value) => Ok(RuntimeType {
            value: RuntimeValue::Boolean(BooleanVal::from(*value)),
            line: expression.line,
            column: expression.column,
        }),
        ast::ExpressionType::Assignment(assignment) => {
            let key = &*assignment.left;
            let value = evaluate_expression(&assignment.right, env)?;

            let key = match &key.expression_type {
                ast::ExpressionType::Identifier(key) => key,
                _ => return Err(InvalidAssignment::new(expression.line, expression.column).into()),
            };

            env.set(key, value.value.clone(), false);

            Ok(value)
        }
        ast::ExpressionType::Binary(binary) => {
            let left = evaluate_expression(&binary.left, env)?;
            let right = evaluate_expression(&binary.right, env)?;

            let operator = &binary.operator;

            match left.value.operation(&right.value, operator.clone()) {
                Ok(value) => Ok(RuntimeType {
                    value,
                    line: expression.line,
                    column: expression.column,
                }),
                Err(error) => match error {
                    BinaryOperationError::InvalidOperation => {
                        Err(InvalidOperation::new(left, right, operator.clone()).into())
                    }
                    BinaryOperationError::InvalidOperationType => {
                        Err(InvalidOperationType::new(left, Some(right), operator.clone()).into())
                    }
                    BinaryOperationError::InvalidOperator => Err(InvalidOperator::new(
                        operator.clone(),
                        expression.line,
                        expression.column,
                    )
                    .into()),
                },
            }
        }
        ast::ExpressionType::Unary(unary) => {
            let raw_value = evaluate_expression(&unary.right, env)?;
            let operator = &unary.operator;

            // ensure value is a number
            let value = match &raw_value.value {
                RuntimeValue::Number(value) => Some(value),
                _ => None,
            };

            if value.is_some() {
                let value = value.unwrap();
                return match operator {
                    Operator::Arithmatic(Arithmatic::Minus) => Ok(RuntimeType {
                        value: RuntimeValue::Number((-value.value).into()),
                        line: expression.line,
                        column: expression.column,
                    }),
                    Operator::Arithmatic(Arithmatic::Plus) => Ok(RuntimeType {
                        value: RuntimeValue::Number((value.value).into()),
                        line: expression.line,
                        column: expression.column,
                    }),
                    _ => Err(InvalidOperator::new(
                        operator.clone(),
                        expression.line,
                        expression.column,
                    )
                    .into()),
                };
            }

            let value = match &raw_value.value {
                RuntimeValue::Boolean(value) => Some(value),
                _ => None,
            };

            if value.is_some() {
                let value = value.unwrap();
                return match operator {
                    Operator::Logical(Logical::Not) => Ok(RuntimeType {
                        value: RuntimeValue::Boolean((!value.value).into()),
                        line: expression.line,
                        column: expression.column,
                    }),
                    _ => Err(InvalidOperator::new(
                        operator.clone(),
                        expression.line,
                        expression.column,
                    )
                    .into()),
                };
            }

            Err(InvalidOperationType::new(raw_value, None, operator.clone()).into())
        }
        ast::ExpressionType::If(cond) => {
            let condition = evaluate_expression(&cond.condition, env)?;

            let condition = match &condition.value {
                RuntimeValue::Boolean(value) => value,
                _ => {
                    return Err(InvalidCondition::new(
                        condition,
                        expression.line,
                        expression.column,
                    )
                    .into())
                }
            };

            if condition.value {
                let mut last_value = RuntimeValue::Null;
                for expression in &cond.body {
                    last_value = evaluate_expression(expression, env)?.value;
                }
                return Ok(RuntimeType {
                    value: last_value,
                    line: expression.line,
                    column: expression.column,
                });
            } else if let Some(else_body) = &cond.else_body {
                let mut last_value = RuntimeValue::Null;
                for expression in else_body {
                    last_value = evaluate_expression(expression, env)?.value;
                }
                return Ok(RuntimeType {
                    value: last_value,
                    line: expression.line,
                    column: expression.column,
                });
            }

            Ok(RuntimeType {
                value: RuntimeValue::Null,
                line: expression.line,
                column: expression.column,
            })
        }
        ast::ExpressionType::Grouping(group) => {
            let value = evaluate_expression(&group.expression, env)?;

            Ok(RuntimeType {
                value: value.value,
                line: expression.line,
                column: expression.column,
            })
        }
    }
}
