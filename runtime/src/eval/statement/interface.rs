use tsr_lexer::globals::Positioned;
use tsr_parser::ast::{InterfaceDeclaration, PropertyName, TypeMember};

use crate::{
    value::{
        CallSignature, ConstructSignature, IndexSignature, MethodSignature, PropertySignature,
        Signature, Value,
    },
    Runtime,
};

impl Runtime {
    pub fn declare_interface(&mut self, interface: Positioned<InterfaceDeclaration>) -> Value {
        let (span, interface) = interface.unpack();
        let value = Value::Interface {
            name: interface.name.value.0.clone(),
            extends: interface
                .extends
                .into_iter()
                .map(|extend| extend.value.0)
                .collect(),
            signatures: interface
                .members
                .into_iter()
                .map(|member| match member.value {
                    TypeMember::PropertySignature(signature) => {
                        Signature::Property(PropertySignature {
                            name: signature.value.name.value.0,
                            nullable: signature.value.nullable.value,
                            ty: signature.value.ty.value,
                        })
                    }
                    TypeMember::CallSignature(signature) => Signature::Call(CallSignature {
                        parameters: signature
                            .value
                            .1
                            .into_iter()
                            .map(|parameter| PropertySignature {
                                name: parameter.value.name.value.0,
                                nullable: parameter.value.nullable.value,
                                ty: parameter.value.ty.value,
                            })
                            .collect(),
                        ty: signature.value.2.value,
                    }),
                    TypeMember::ConstructSignature(signature) => {
                        Signature::Construct(ConstructSignature {
                            parameters: signature
                                .value
                                .1
                                .into_iter()
                                .map(|parameter| PropertySignature {
                                    name: parameter.value.name.value.0,
                                    nullable: parameter.value.nullable.value,
                                    ty: parameter.value.ty.value,
                                })
                                .collect(),
                            ty: signature.value.2.value,
                        })
                    }
                    TypeMember::IndexSignature(signature) => Signature::Index(IndexSignature {
                        name: signature.value.0.value.0,
                        index_type: signature.value.1.value,
                        ty: signature.value.2.value,
                    }),
                    TypeMember::MethodSignature(signature) => Signature::Method(MethodSignature {
                        name: match signature.value.0.value {
                            PropertyName::LiteralPropertyName(literal) => {
                                self.eval_literal(literal.value)
                            }
                            PropertyName::ComputedPropertyName(expression) => {
                                self.eval_expression(expression)
                            }
                        },
                        parameters: signature
                            .value
                            .2
                            .value
                            .1
                            .into_iter()
                            .map(|parameter| PropertySignature {
                                name: parameter.value.name.value.0,
                                nullable: parameter.value.nullable.value,
                                ty: parameter.value.ty.value,
                            })
                            .collect(),
                        ty: signature.value.2.value.2.value,
                    }),
                })
                .collect(),
        };

        self.set_variable(interface.name.value.0, span.wrap(value))
    }
}
