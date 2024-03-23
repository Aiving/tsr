use tsr_lexer::{globals::Positioned, token::Modifier};
use tsr_parser::ast::FunctionDeclaration;

use crate::{value::{Function, Parameter, Value, Visibility}, Runtime};

impl Runtime {
    pub fn declare_function(&mut self, function: Positioned<FunctionDeclaration>) -> Value {
        let (span, function) = function.unpack();
        let mut visibility = Visibility::Private;
        let mut is_async = false;
        let mut is_static = false;

        for modifier in function.modifiers {
            let modifier = modifier.value;

            match modifier {
                Modifier::Public => visibility = Visibility::Public,
                Modifier::Private => visibility = Visibility::Private,
                Modifier::Protected => visibility = Visibility::Protected,
                Modifier::Async => is_async = true,
                Modifier::Static => is_static = true,
            }
        }

        if let Some(body) = function.body {
            let parameters = function
                .parameters
                .into_iter()
                .map(|param| Parameter {
                    name: param.value.name.value.0,
                    nullable: param.value.nullable.value,
                    ty: param.value.ty.value,
                    default: param
                        .value
                        .default
                        .map(|expression| Box::new(self.eval_expression(expression))),
                })
                .collect();

            self.set_variable(
                function.name.value.0.clone(),
                span.wrap(Value::Function(Function {
                    visibility,
                    overloads: Vec::default(),
                    is_async,
                    is_static,
                    name: function.name.value.0,
                    parameters,
                    ty: function.ty.value,
                    body,
                })),
            );
        }

        Value::None
    }
}