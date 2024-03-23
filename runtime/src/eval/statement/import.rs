use tsr_lexer::globals::Positioned;
use tsr_parser::ast::{ImportClause, ImportDeclaration, Literal};

use crate::{value::{builders::ObjectBuilder, ErrorCode, Value}, Runtime};

impl Runtime {
    pub fn eval_import(&mut self, import: Positioned<ImportDeclaration>) -> Value {
        let (span, import) = (import.span, import.value);

        let Literal::String(module) = import.module_specifier.value else {
            unreachable!()
        };

        if let Some(module) = self
            .modules
            .iter()
            .find(|native| native.name == module.value)
        {
            if let Some(clause) = import.import_clause {
                match clause.value {
                    ImportClause::Named(_) => {}
                    ImportClause::NamedImports(value) => {
                        for specifier in value {
                            let specifier = specifier.value;
                            let (name_span, name) = specifier
                                .property_name
                                .map(|alias| alias.unpack())
                                .unwrap_or(specifier.name.clone().unpack());

                            if let Some(value) = module
                                .exports
                                .iter()
                                .find(|export| export.0 == specifier.name.value.0)
                            {
                                self.set_variable(name.0, span.wrap(value.1.clone()));
                            } else {
                                self.error = Some(Value::error(
                                    name_span,
                                    ErrorCode::Reference,
                                    format!(
                                        "found \"{}\" while excepted one of next values: {}",
                                        specifier.name.value,
                                        module
                                            .exports
                                            .iter()
                                            .map(|specifier| specifier.0.as_str())
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    ),
                                ));

                                break;
                            }
                        }
                    }
                    ImportClause::NamespaceImport(value) => {
                        let mut object = ObjectBuilder::default();

                        for (key, value) in &module.exports {
                            object = object.prop(key, value.clone());
                        }

                        self.set_variable(value.value.0, value.span.wrap(object.build()));
                    }
                }
            }
        } else {
            self.error = Some(Value::error(
                module.span,
                ErrorCode::Reference,
                format!("failed to resolve module \"{}\"", module.value),
            ));
        }

        Value::None
    }
}
