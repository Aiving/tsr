use tsr_lexer::token::Operator;
use tsr_parser::ast::BinaryExpression;

use crate::{value::Value, Runtime};

impl Runtime {
    pub fn eval_binary_expression(&mut self, expression: BinaryExpression) -> Value {
        let left = self.eval_expression(expression.left);
        let right = self.eval_expression(expression.right);

        println!("{left} {:?} {right}", expression.operator.value);

        match expression.operator.value {
            Operator::And => todo!(),
            Operator::AndAnd => todo!(),
            Operator::Plus => match (left, right) {
                (Value::String(first), Value::String(second)) => Value::String(first + &second),
                (Value::Number(first), Value::Number(second)) => Value::Number(first + second),
                (_, _) => todo!(),
            },
            Operator::Star => match (left, right) {
                (Value::String(data), Value::Number(times)) => {
                    Value::String(data.repeat(times as usize))
                }
                (Value::Number(first), Value::Number(second)) => Value::Number(first * second),
                (_, _) => todo!(),
            },
            Operator::Slash => match (left, right) {
                (Value::Number(first), Value::Number(second)) => Value::Number(first / second),
                (_, _) => todo!(),
            },
            Operator::Or => todo!(),
            Operator::OrOr => todo!(),
            Operator::PlusPlus => match left {
                Value::Reference(path, scope) => {
                    let mut context = self.context.lock().unwrap();

                    if let Some(Value::Number(value)) =
                        context.get(&path[0], scope.clone()).map(|var| &var.value)
                    {
                        let value = Value::Number(value + 1);

                        context.set(&path, scope, value);
                    }

                    Value::None
                }
                _ => todo!(),
            },
            Operator::Minus => match (left, right) {
                (Value::Number(first), Value::Number(second)) => Value::Number(first - second),
                (_, _) => todo!(),
            },
            Operator::MinusMinus => match left {
                Value::Reference(path, scope) => {
                    let mut context = self.context.lock().unwrap();

                    if let Some(Value::Number(value)) =
                        context.get(&path[0], scope.clone()).map(|var| &var.value)
                    {
                        let value = Value::Number(value - 1);

                        context.set(&path, scope, value);
                    }

                    Value::None
                }
                _ => todo!(),
            },
            Operator::EqEq => Value::Boolean(left == right),
            Operator::Eq => match (left, right) {
                (Value::Reference(path, scope), value) => {
                    let value = match value {
                        Value::Reference(path, scope) => todo!(),
                        value => value,
                    };

                    self.context.lock().unwrap().set(&path, scope, value);

                    Value::None
                }
                (_, _) => todo!(),
            },
            Operator::Ne => Value::Boolean(left != right),
            Operator::Le => todo!(),
            Operator::Ge => todo!(),
            Operator::Lt => todo!(),
            Operator::Gt => todo!(),
            Operator::Not => match left {
                Value::Reference(path, scope) => {
                    if let Some(Value::Boolean(value)) = self
                        .context
                        .lock()
                        .unwrap()
                        .get(&path[0], scope.clone())
                        .map(|var| &var.value)
                    {
                        return Value::Boolean(!value);
                    }

                    Value::None
                }
                Value::Boolean(value) => Value::Boolean(!value),
                _ => todo!(),
            },
        }
    }
}