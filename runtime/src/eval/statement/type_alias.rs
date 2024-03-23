use tsr_lexer::globals::Positioned;
use tsr_parser::ast::TypeAliasDeclaration;

use crate::{value::Value, Runtime};

impl Runtime {
    pub fn declare_type_alias(&mut self, type_alias: Positioned<TypeAliasDeclaration>) -> Value {
        let (span, type_alias) = type_alias.unpack();
        let value = Value::TypeAlias {
            name: type_alias.name.value.0.clone(),
            ty: type_alias.ty.value,
        };

        self.set_variable(type_alias.name.value.0, span.wrap(value))
    }
}
