use tsr_parser::ast::PredefinedType;

use crate::value::native::Module;
use crate::value::native::NativeModule;
use crate::value::Value;
use crate::FArguments;
use crate::FunctionBuilder;

pub struct Util;

#[tsr_macro::native_module("util")]
impl Util {
    #[func(args = [("value", PredefinedType::Any), ("pretty", PredefinedType::Boolean, false)], returns = PredefinedType::String)]
    fn inspect(&self, args: &FArguments) -> Option<impl Into<Value>> {
        let pretty = args.get_boolean("pretty");

        args.get("value").map(|value| match pretty {
            true => format!("{value:#}"),
            false => format!("{value}"),
        })
    }
}
