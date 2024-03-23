use std::collections::HashMap;
use std::sync::Arc;

use crate::value::NativeFunction;
use crate::value::Parameter;
use crate::value::Visibility;
use crate::FArguments;
use crate::Type;
use super::Value;

#[derive(Default)]
pub struct ObjectBuilder {
    properties: HashMap<Value, Value>,
}

impl ObjectBuilder {
    pub fn prop<N: AsRef<str>, V: Into<Value>>(mut self, name: N, value: V) -> Self {
        let name: &str = name.as_ref();

        self.properties
            .insert(Value::String(name.into()), value.into());

        self
    }

    pub fn build(self) -> Value {
        Value::Object(self.properties)
    }
}

pub struct FunctionBuilder {
    visibility: Visibility,
    name: String,
    parameters: Vec<Parameter>,
    ty: Type,
}

impl FunctionBuilder {
    pub fn new<N: AsRef<str>>(name: N) -> Self {
        let name: &str = name.as_ref();

        FunctionBuilder {
            visibility: Default::default(),
            name: name.into(),
            parameters: Default::default(),
            ty: Default::default(),
        }
    }

    pub fn param<N: AsRef<str>, T: Into<Type>>(mut self, name: N, ty: T) -> Self {
        let name: &str = name.as_ref();

        self.parameters.push(Parameter {
            name: name.into(),
            nullable: false,
            ty: ty.into(),
            default: None,
        });

        self
    }

    pub fn param_default<N: AsRef<str>, T: Into<Type>, V: Into<Value>>(mut self, name: N, ty: T, default: V) -> Self {
        let name: &str = name.as_ref();

        self.parameters.push(Parameter {
            name: name.into(),
            nullable: false,
            ty: ty.into(),
            default: Some(Box::new(default.into())),
        });

        self
    }

    pub fn returns<T: Into<Type>>(mut self, ty: T) -> Self {
        self.ty = ty.into();

        self
    }

    pub fn build<F: Fn(&mut FArguments) + 'static>(self, body: F) -> Value {
        Value::NativeFunction(NativeFunction {
            visibility: self.visibility,
            ty: self.ty,
            name: self.name,
            parameters: self.parameters,
            body: Arc::new(body),
        })
    }
}
