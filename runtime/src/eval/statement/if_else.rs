use tsr_lexer::globals::Positioned;
use tsr_parser::ast::IfStatement;

use crate::{value::Value, Runtime};

impl Runtime {
    pub fn eval_if(&mut self, statement: Positioned<IfStatement>) -> Value {
        Value::None
    }
}
