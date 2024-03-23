use tsr_lexer::globals::Positioned;
use tsr_parser::ast::NewExpression;

use crate::{value::{ClassInstance, ErrorCode, Field, Value}, Runtime};

impl Runtime {
    pub fn eval_new_expression(&mut self, expression: Positioned<NewExpression>) -> Value {
        let (span, expression) = expression.unpack();
        let arguments = expression
            .arguments
            .into_iter()
            .map(|argument| (argument.span, self.eval_expression(argument)))
            .collect::<Vec<_>>();

        let class_name = self.eval_expression(*expression.expression);

        if let Value::Reference(path, scope) = class_name {
            let class = self
                .context
                .lock()
                .unwrap()
                .get(&path[0], scope)
                .map(|variable| variable.value.clone());

            if let Some(Value::Class {
                name,
                constructors,
                fields,
                ..
            }) = class
            {
                let constructor = constructors.into_iter().find(|constructor| {
                    let arg_count = constructor
                        .parameters
                        .iter()
                        .filter(|parameter| parameter.default.is_none())
                        .count();

                    arguments.len() >= arg_count
                        && arguments.iter().enumerate().all(|(index, (_, argument))| {
                            constructor
                                .parameters
                                .get(index)
                                .is_some_and(|parameter| argument.is_type_of(&parameter.ty))
                        })
                });

                if let Some(constructor) = constructor {
                    self.add_scope(format!("class-instance:{name}"));

                    self.set_variable(
                        "this",
                        span.wrap(Value::ClassInstance(ClassInstance {
                            name,
                            fields: fields
                                .into_iter()
                                .map(|prop| {
                                    let value =
                                        prop.init.clone().map_or(Value::None, |value| *value);

                                    Field { prop, value }
                                })
                                .collect(),
                        })),
                    );

                    constructor.call(span, self, arguments, None);

                    if let Some(Value::ClassInstance(instance)) = self
                        .context
                        .lock()
                        .unwrap()
                        .get("this", self.scope.clone())
                        .map(|var| &var.value)
                    {
                        let mut error = None;

                        for field in &instance.fields {
                            if field.value.is_none()
                                && !field.prop.nullable
                                && field.prop.init.is_none()
                            {
                                error = Some(Value::error(
                                    span,
                                    ErrorCode::Declaration,
                                    format!(
                                        "\"{}\" has not been declared in constructor",
                                        field.prop.name
                                    ),
                                ));

                                break;
                            }
                        }

                        if error.is_some() {
                            self.error = error;

                            return Value::None;
                        }
                    }

                    let this = Value::Reference(vec!["this".into()], self.scope.clone());

                    self.remove_scope();

                    return this;
                }
            }
        }

        Value::None
    }
}