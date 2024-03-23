use tsr_lexer::{globals::Positioned, token::Modifier};
use tsr_parser::ast::{
    ClassDeclaration, ClassElement, Literal, PrimaryType, PropertyMemberDeclaration, PropertyName,
};

use crate::{
    value::{Function, Parameter, Property, Value, Visibility},
    Runtime,
};

impl Runtime {
    pub fn declare_class(&mut self, class: Positioned<ClassDeclaration>) -> Value {
        let (span, class) = class.unpack();
        let name = class.name.value.0;
        let extends = class
            .extends
            .into_iter()
            .map(|extend| extend.value.0)
            .collect();
        let implements = class
            .implements
            .into_iter()
            .map(|implement| implement.value.0)
            .collect();

        let mut constructors = Vec::new();
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        for element in class.body {
            let element = element.value;

            match element {
                ClassElement::ConstructorDeclaration(declaration) => {
                    let declaration = declaration.value;

                    let mut visibility = Visibility::Private;
                    let mut is_async = false;
                    let mut is_static = false;

                    for modifier in declaration.modifiers {
                        let modifier = modifier.value;

                        match modifier {
                            Modifier::Public => visibility = Visibility::Public,
                            Modifier::Private => visibility = Visibility::Private,
                            Modifier::Protected => visibility = Visibility::Protected,
                            Modifier::Async => is_async = true,
                            Modifier::Static => is_static = true,
                        }
                    }

                    constructors.push(Function {
                        visibility,
                        overloads: Vec::new(),
                        is_async,
                        is_static,
                        name: "constructor".into(),
                        parameters: declaration
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
                            .collect(),
                        ty: PrimaryType::ThisType.into(),
                        body: declaration.body,
                    });
                }
                ClassElement::PropertyMemberDeclaration(declaration) => match declaration.value {
                    PropertyMemberDeclaration::MemberVariableDeclaration(declaration) => {
                        let declaration = declaration.value;
                        let init = declaration
                            .initializer
                            .map(|expression| Box::new(self.eval_expression(expression)));

                        fields.push(Property {
                            name: match declaration.name.value {
                                PropertyName::LiteralPropertyName(literal) => match literal.value {
                                    Literal::String(string) => string.value,
                                    _ => todo!(),
                                },
                                PropertyName::ComputedPropertyName(_) => todo!(),
                            },
                            nullable: false,
                            ty: declaration.ty.map_or_else(
                                || init.as_ref().unwrap().value_type_of(),
                                |ty| ty.value,
                            ),
                            init,
                        })
                    }
                    PropertyMemberDeclaration::MemberFunctionDeclaration(declaration) => {
                        let declaration = declaration.value;

                        let mut visibility = Visibility::Private;
                        let mut is_async = false;
                        let mut is_static = false;

                        for modifier in declaration.modifiers {
                            let modifier = modifier.value;

                            match modifier {
                                Modifier::Public => visibility = Visibility::Public,
                                Modifier::Private => visibility = Visibility::Private,
                                Modifier::Protected => visibility = Visibility::Protected,
                                Modifier::Async => is_async = true,
                                Modifier::Static => is_static = true,
                            }
                        }

                        methods.push(Function {
                            visibility,
                            overloads: Vec::new(),
                            is_async,
                            is_static,
                            name: match declaration.name.value {
                                PropertyName::LiteralPropertyName(literal) => match literal.value {
                                    Literal::String(string) => string.value,
                                    _ => todo!(),
                                },
                                PropertyName::ComputedPropertyName(_) => todo!(),
                            },
                            parameters: declaration
                                .parameters
                                .into_iter()
                                .map(|param| Parameter {
                                    name: param.value.name.value.0,
                                    nullable: param.value.nullable.value,
                                    ty: param.value.ty.value,
                                    default: param.value.default.map(|expression| {
                                        Box::new(self.eval_expression(expression))
                                    }),
                                })
                                .collect(),
                            ty: declaration.ty.value,
                            body: declaration.body,
                        });
                    }
                    PropertyMemberDeclaration::MemberAccessorDeclaration(_) => todo!(),
                },
                ClassElement::IndexMemberDeclaration(_) => todo!(),
            }
        }

        self.set_variable(
            name.clone(),
            span.wrap(Value::Class {
                name,
                extends,
                implements,
                constructors,
                fields,
                methods,
            }),
        )
    }
}
