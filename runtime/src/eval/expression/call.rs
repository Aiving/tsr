use tsr_lexer::globals::Positioned;
use tsr_parser::ast::{FunctionCallExpression, Type};

use crate::{
    value::{self, ArrowParameter, ErrorCode, NativeFunction, Value}, FArguments, Runtime
};

impl Runtime {
    pub fn eval_call(&mut self, call: Positioned<FunctionCallExpression>) -> Value {
        let (span, call) = call.unpack();
        let func = match self.eval_expression(*call.function) {
            Value::Reference(path, scope) => match todo!() {
                Value::Reference(path, scope) => self
                    .context
                    .lock()
                    .unwrap()
                    .get(&path[0], scope.clone())
                    .unwrap()
                    .value
                    .clone(),
                value => value,
            },
            value => value,
        };
        let mut args = call
            .arguments
            .into_iter()
            .map(|argument| {
                (
                    argument.span,
                    match self.eval_expression(argument) {
                        Value::Reference(path, scope) => todo!(),
                        value => value,
                    },
                )
            })
            .collect::<Vec<_>>();

        if let Some(lambda) = call.lambda.as_ref() {
            args.push((lambda.span, Value::None));
        }

        if matches!(func, Value::Error { .. }) {
            return func.clone();
        }

        match &func {
            Value::NativeFunction(NativeFunction {
                name,
                parameters,
                body,
                ty,
                ..
            }) => {
                let arg_count = parameters
                    .iter()
                    .filter(|parameter| parameter.default.is_none())
                    .count();

                if args.len() < arg_count {
                    return Value::error(
                        span,
                        ErrorCode::Type,
                        format!(
                            "wrong number of arguments: {} expected but {} given",
                            arg_count,
                            args.len()
                        ),
                    );
                }

                let zipped = parameters.iter().zip(args);

                self.add_scope(format!("func:{}", name));

                for (argument, (span, value)) in zipped {
                    if value.is_none() && call.lambda.is_some() {
                        if let Type::FunctionType(_, params, ty) = argument.ty.clone() {
                            let value = span.wrap(Value::ArrowFunction(value::ArrowFunction {
                                is_async: false,
                                parameters: params
                                    .into_iter()
                                    .map(|param| ArrowParameter {
                                        name: param.name.value.0,
                                        nullable: param.nullable.value,
                                        ty: Some(param.ty.value),
                                        default: param
                                            .default
                                            .map(|default| Box::new(self.eval_expression(default))),
                                    })
                                    .collect(),
                                ty: Some(*ty),
                                body: call.lambda.unwrap(),
                            }));

                            self.set_variable(&argument.name, value);

                            break;
                        }
                    } else {
                        if !value.is_type_of(&argument.ty) {
                            return Value::error(
                                span,
                                ErrorCode::Type,
                                format!(
                                    "{} expected but {} given",
                                    argument.ty,
                                    value.value_type_of()
                                ),
                            );
                        }

                        self.set_variable(&argument.name, span.wrap(value));
                    }
                }

                let mut args = FArguments {
                    context: self.get_context(),
                    scope: self.scope.clone(),
                    returns: None,
                };

                (body)(&mut args);

                self.clear_scope_variables();
                self.remove_scope();

                if let Some(value) = args.returns {
                    if value.is_type_of(&ty) {
                        return value;
                    }
                }
            }
            Value::Function(func) => match func.call(span, self, args, call.lambda) {
                Value::Error(span, code, message) => {
                    self.error = Some(Value::Error(span, code, message))
                }
                value => return value,
            },
            Value::ArrowFunction(func) => {
                let arg_count = func
                    .parameters
                    .iter()
                    .filter(|parameter| parameter.default.is_none())
                    .count();

                if args.len() < arg_count {
                    return Value::error(
                        span,
                        ErrorCode::Type,
                        format!(
                            "wrong number of arguments: {} expected but {} given",
                            arg_count,
                            args.len()
                        ),
                    );
                }

                let zipped = func.parameters.iter().zip(args);

                self.add_scope("closure");

                for (argument, (span, value)) in zipped {
                    if argument.ty.as_ref().is_some_and(|ty| !value.is_type_of(ty)) {
                        return Value::error(
                            span,
                            ErrorCode::Type,
                            format!(
                                "{} expected but {} given",
                                argument.ty.as_ref().unwrap(),
                                value.value_type_of()
                            ),
                        );
                    }

                    self.set_variable(&argument.name, span.wrap(value));
                }

                let value = self.eval_code_block(func.body.clone());

                self.clear_scope_variables();
                self.remove_scope();

                if func.ty.is_none() || func.ty.as_ref().is_some_and(|ty| value.is_type_of(ty)) {
                    return value;
                }
            }
            _ => {}
        }

        Value::None
    }
}
