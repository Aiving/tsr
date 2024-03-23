pub use self::value::builders::FunctionBuilder;
use self::{
    environment::{Context, Environment, Scope},
    value::{
        native::{Module, NativeModule},
        ErrorCode, Function, NativeFunction, Parameter, Signature, Value, Visibility,
    },
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tsr_lexer::globals::Positioned;
use tsr_parser::ast::{Block, Type};

pub mod api;
pub mod environment;
pub mod eval;
pub mod value;

pub type FunctionTuple = (Visibility, bool, bool, bool, String, Vec<Parameter>, Type);

macro_rules! try_unpack {
    ($variant:path, $value:expr) => {
        if let Some($variant(x)) = $value {
            Some(x)
        } else {
            None
        }
    };
}

#[derive(Debug, Clone)]
pub struct FArguments {
    context: Context,
    scope: Scope,
    returns: Option<Value>,
}

impl FArguments {
    fn get_by_scope<N: AsRef<str>>(&self, name: N, scope: Scope) -> Option<Value> {
        let value = {
            let context = self.context.lock().unwrap();

            context
                .get(name, scope)
                .map(|variable| variable.value.clone())
        };

        match value {
            Some(Value::Reference(name, scope)) if name.len() == 1 => {
                self.get_by_scope(&name[0], scope)
            }
            value => value,
        }
    }

    pub fn get<N: AsRef<str>>(&self, name: N) -> Option<Value> {
        self.get_by_scope(name, self.scope.clone())
    }

    pub fn returns<V: Into<Value>>(&mut self, value: V) {
        self.returns = Some(value.into());
    }

    pub fn get_interface<N: AsRef<str>>(
        &self,
        name: N,
    ) -> Option<(String, Vec<String>, Vec<Signature>)> {
        if let Some(Value::Interface {
            name,
            extends,
            signatures,
        }) = self.get(name)
        {
            Some((name, extends, signatures))
        } else {
            None
        }
    }

    pub fn get_function<N: AsRef<str>>(&self, name: N) -> Option<FunctionTuple> {
        if let Some(value) = self.get(name) {
            match value {
                Value::NativeFunction(NativeFunction {
                    visibility,
                    name,
                    parameters,
                    ty,
                    ..
                }) => Some((visibility, false, false, true, name, parameters, ty)),
                Value::Function(Function {
                    visibility,
                    is_async,
                    is_static,
                    name,
                    parameters,
                    ty,
                    ..
                }) => Some((visibility, is_async, is_static, false, name, parameters, ty)),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_number<N: AsRef<str>>(&self, name: N) -> i64 {
        try_unpack!(Value::Number, self.get(name)).unwrap()
    }

    pub fn get_boolean<N: AsRef<str>>(&self, name: N) -> bool {
        try_unpack!(Value::Boolean, self.get(name)).unwrap()
    }

    pub fn get_string<N: AsRef<str>>(&self, name: N) -> String {
        try_unpack!(Value::String, self.get(name)).unwrap()
    }

    pub fn get_number_opt<N: AsRef<str>>(&self, name: N) -> Option<i64> {
        try_unpack!(Value::Number, self.get(name))
    }

    pub fn get_boolean_opt<N: AsRef<str>>(&self, name: N) -> Option<bool> {
        try_unpack!(Value::Boolean, self.get(name))
    }

    pub fn get_string_opt<N: AsRef<str>>(&self, name: N) -> Option<String> {
        try_unpack!(Value::String, self.get(name))
    }
}

#[derive(Debug, Clone)]
pub struct Runtime {
    types: Arc<RwLock<HashMap<String, Type>>>,
    context: Context,
    modules: Vec<Module>,
    error: Option<Value>,
    scope: Scope,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            types: Arc::new(RwLock::new(HashMap::new())),
            context: Environment::new(),
            modules: Default::default(),
            scope: vec!["root".into()],
            error: None,
        }
    }
}

impl Runtime {
    pub fn new(context: Context) -> Self {
        Self {
            types: Arc::new(RwLock::new(HashMap::new())),
            context,
            modules: Default::default(),
            scope: vec!["root".into()],
            error: None,
        }
    }

    fn add_scope<S: AsRef<str>>(&mut self, scope: S) {
        let scope: &str = scope.as_ref();

        self.scope.push(scope.into());
    }

    fn remove_scope(&mut self) {
        self.scope.pop();
    }

    fn clear_scope_variables(&mut self) {
        self.context
            .lock()
            .unwrap()
            .remove_by_scope(self.scope.clone());
    }

    fn clear_scope_variables_filtered(&mut self, variables: &[String]) {
        self.context
            .lock()
            .unwrap()
            .remove_by_scope_filtered(self.scope.clone(), variables);
    }

    // fn associate_type(&self, name: String, ty: Type) {
    //     self.types.write().unwrap().insert(name, ty);
    // }

    // fn resolve_type(&self, name: String) -> Option<Type> {
    //     self.types.read().unwrap().get(&name).cloned()
    // }

    pub fn add_module<M: NativeModule>(&mut self, module: &'static M) {
        self.modules.push(module.build_module());
    }

    pub fn set_variable<N: AsRef<str>>(&self, name: N, value: Positioned<Value>) -> Value {
        let (span, value) = value.unpack();
        let name = [name.as_ref()];

        if let Value::Error { .. } = value {
            return value;
        }

        let mut context = self.context.lock().unwrap();

        if let Some(Value::Function(function)) = context
            .get_mut(name[0], self.scope.clone())
            .map(|variable| &mut variable.value)
        {
            if let Value::Function(func) = value {
                function.overloads.push(func);
            } else {
                return Value::error(
                    span,
                    ErrorCode::Type,
                    format!("Function expected, but {} given", value.type_of()),
                );
            }
        } else {
            context.set(&name, self.scope.clone(), value);
        }

        Value::None
    }

    fn eval_code_block(&mut self, mut block: Block) -> Value {
        match block.value.len() {
            0 => Value::None,
            1 => self.eval_statement(block.value.remove(0)),
            _ => {
                let statement = block.value.remove(0);
                let value = self.eval_statement(statement);

                if let Some(error) = &self.error {
                    error.clone()
                } else if value.is_returned() {
                    value
                } else {
                    self.eval_code_block(block)
                }
            }
        }
    }

    pub fn get_context(&self) -> Context {
        self.context.clone()
    }

    fn returned(&self, value: Value) -> Value {
        match value {
            Value::ReturnValue(value) => *value,
            _ => value,
        }
    }

    pub fn eval_program(&mut self, program: Block) -> Value {
        let value = self.eval_code_block(program);

        self.returned(value)
    }
}

impl Runtime {
    fn is_array(&self, value: Value) -> bool {
        match value {
            Value::Array(..) => true,
            Value::Reference(path, scope) => todo!(),
            _ => false,
        }
    }
}
