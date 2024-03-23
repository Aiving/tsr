use tsr_lexer::token::Modifier;
use tsr_parser::ast::{ArrowFunction, Statement};

use crate::{value::{self, ArrowParameter, Value}, Runtime};

impl Runtime {
    pub fn eval_arrow_function(&mut self, func: ArrowFunction) -> Value {
        Value::ArrowFunction(value::ArrowFunction {
            is_async: func
                .modifiers
                .iter()
                .any(|modifier| modifier.value == Modifier::Async),
            parameters: func
                .parameters
                .into_iter()
                .map(|parameter| ArrowParameter {
                    name: parameter.value.name.value.0,
                    nullable: parameter.value.nullable.value,
                    ty: parameter.value.ty.map(|ty| ty.value),
                    default: parameter
                        .value
                        .default
                        .map(|expression| Box::new(self.eval_expression(expression))),
                })
                .collect(),
            ty: func.ty.map(|ty| ty.value),
            body: func
                .body
                .span
                .wrap(vec![func.body.span.wrap(Statement::Expression(func.body))]),
        })
    }
}