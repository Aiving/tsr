use tsr_lexer::globals::Positioned;
use tsr_parser::ast::ExportDeclaration;

use crate::{value::Value, Runtime};

impl Runtime {
    pub fn eval_export(&mut self, export: Positioned<ExportDeclaration>) -> Value {
        let (span, value) = export.unpack();

        match value {
            ExportDeclaration::Default(statement) => match statement.value {
                tsr_parser::ast::ExportDefaultElement::FunctionDeclaration(declaration) => {
                    self.declare_function(declaration)
                }
                tsr_parser::ast::ExportDefaultElement::ClassDeclaration(_) => todo!(),
                tsr_parser::ast::ExportDefaultElement::Expression(_) => todo!(),
                tsr_parser::ast::ExportDefaultElement::IdentifierReference(_) => todo!(),
            },
            ExportDeclaration::Single(_) => todo!(),
            ExportDeclaration::List(_) => todo!(),
        }
    }
}
