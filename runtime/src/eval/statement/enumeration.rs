use tsr_lexer::globals::Positioned;
use tsr_parser::ast::EnumDeclaration;

use crate::{
    value::{Enum, EnumMember, Value},
    Runtime,
};

impl Runtime {
    pub fn declare_enum(&mut self, enumeration: Positioned<EnumDeclaration>) -> Value {
        let (span, enumeration) = enumeration.unpack();
        let mut members = vec![];

        for (index, member) in enumeration.members.into_iter().enumerate() {
            if let Some(initializer) = member.value.initializer {
                let init = self.eval_expression(initializer);

                if matches!(init, Value::Error { .. }) {
                    return init;
                }

                members.push(EnumMember {
                    name: member.value.name.value.0,
                    init: Box::new(init),
                });
            } else {
                members.push(EnumMember {
                    name: member.value.name.value.0,
                    init: Box::new(Value::Number(index as i64)),
                });
            }
        }
        let value = Value::Enum(Enum {
            name: enumeration.name.value.0.clone(),
            members,
        });

        self.set_variable(enumeration.name.value.0, span.wrap(value))
    }
}
