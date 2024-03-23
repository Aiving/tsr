use std::collections::HashMap;

use tsr_parser::ast::ArraySize;
use tsr_parser::ast::PredefinedType;
use tsr_parser::ast::PrimaryType;

use crate::value::native::Module;
use crate::value::native::NativeModule;
use crate::value::Signature;
use crate::value::Value;
use crate::FArguments;
use crate::FunctionBuilder;
use crate::value::builders::ObjectBuilder;

pub struct Events;

#[tsr_macro::native_module("events")]
impl Events {
    #[class(name = "HelloWorld")]
    fn hello_world() {

    }

    #[func(name = "getType", args = [("value", PredefinedType::Any)], returns = PredefinedType::String)]
    fn get_type(&self, args: &FArguments) -> Option<impl Into<Value>> {
        args.get("data")
            .map(|data| data.value_type_of().to_string())
    }

    #[func(name = "getFunctionData", args = [("function", PredefinedType::Any)], returns = PrimaryType::ObjectType(vec![]))]
    fn get_function_data(&self, args: &FArguments) -> Option<impl Into<Value>> {
        args.get_function("function").map(
            |(_, is_async, is_static, is_native, name, params, ty)| {
                ObjectBuilder::default()
                    .prop("isAsync", is_async)
                    .prop("isStatic", is_static)
                    .prop("isNative", is_native)
                    .prop("name", name)
                    .prop(
                        "arguments",
                        params
                            .into_iter()
                            .map(|param| {
                                ObjectBuilder::default()
                                    .prop("name", param.name)
                                    .prop("nullable", param.nullable)
                                    .prop("type", param.ty.to_string())
                                    .prop("haveDefaultValue", param.default.is_some())
                                    .build()
                            })
                            .collect::<Vec<_>>(),
                    )
                    .prop("returnType", ty.to_string())
                    .build()
            },
        )
    }

    #[func(name = "getInterfaceData", args = [("interface", PredefinedType::Any)], returns = PrimaryType::ObjectType(vec![]))]
    fn get_interface_data(&self, args: &FArguments) -> Option<impl Into<Value>> {
        args.get_interface("interface")
            .map(|(name, extends, signatures)| {
                ObjectBuilder::default()
                    .prop("name", name)
                    .prop("extends", extends)
                    .prop(
                        "signatures",
                        Value::Array(
                            signatures
                                .iter()
                                .map(|signature| match signature {
                                    // Signature::Property(_) => Value::Object(HashMap::default()),
                                    // Signature::Call(_) => Value::String("todo".into()),
                                    // Signature::Construct(_) => Value::String("todo".into()),
                                    // Signature::Index(_) => Value::String("todo".into()),
                                    Signature::Method(signature) => ObjectBuilder::default()
                                        .prop(
                                            "name",
                                            match &signature.name {
                                                Value::String(value) => value.clone(),
                                                value => value.to_string(),
                                            },
                                        )
                                        .prop(
                                            "parameters",
                                            signature
                                                .parameters
                                                .iter()
                                                .map(|parameter| {
                                                    ObjectBuilder::default()
                                                        .prop("name", parameter.name.clone())
                                                        .prop("nullable", parameter.nullable)
                                                        .prop("type", parameter.ty.to_string())
                                                        .build()
                                                })
                                                .collect::<Vec<_>>(),
                                        )
                                        .prop("returnType", signature.ty.to_string())
                                        .build(),
                                    _ => Value::Object(HashMap::default()),
                                })
                                .collect(),
                            ArraySize::Dynamic,
                        ),
                    )
                    .build()
            })
    }
}
