pub mod environment;

use crate::{
    errors::*,
    parser::ast,
    types::{
        boolean::BooleanVal, character::CharacterVal, number::NumberVal, string::StringVal,
        BinaryOperation, BinaryOperationError, RuntimeType, RuntimeValue,
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
                        Err(InvalidOperationType::new(left, right, operator.clone()).into())
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
        ast::ExpressionType::Unary(_) => todo!(),
        ast::ExpressionType::Grouping(_) => todo!(),
    }
}
