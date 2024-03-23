use tsr_lexer::globals::Positioned;
use tsr_parser::ast::Statement;

use crate::{value::Value, Runtime};

pub mod class;
pub mod enumeration;
pub mod export;
pub mod function;
pub mod if_else;
pub mod import;
pub mod interface;
pub mod type_alias;
pub mod variable;

impl Runtime {
    pub fn eval_statement(&mut self, statement: Positioned<Statement>) -> Value {
        match statement.value {
            Statement::ImportDeclaration(import) => self.eval_import(*import),
            Statement::TypeAliasDeclaration(type_alias) => self.declare_type_alias(type_alias),
            Statement::InterfaceDeclaration(interface) => self.declare_interface(interface),
            Statement::FunctionDeclaration(function) => self.declare_function(function),
            Statement::EnumDeclaration(enumeration) => self.declare_enum(enumeration),
            Statement::ExportDeclaration(export) => self.eval_export(export),
            Statement::ClassDeclaration(class) => self.declare_class(class),
            Statement::VariableStatement(variable) => self.declare_variable(variable),
            Statement::IfStatement(statement) => self.eval_if(*statement),
            Statement::ReturnStatement(statement) => self.eval_expression(statement),
            Statement::Expression(expression) => self.eval_expression(expression),
        }
    }
}
