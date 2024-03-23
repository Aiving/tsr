pub mod builders;
pub mod native;

use super::{environment::Scope, FArguments, Runtime};
use owo_colors::{colors, Color};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
};
use tsr_lexer::globals::Span;
use tsr_parser::ast::{
    ArraySize, Block, IntersectionOrPrimaryType, Literal, PredefinedType, PrimaryType, Type,
    UnionOrIntersectionOrPrimaryType,
};

pub trait Args {
    fn formatted(&self) -> Vec<String>;
}

impl<A: Display> Args for (A,) {
    fn formatted(&self) -> Vec<String> {
        vec![self.0.to_string()]
    }
}

impl<A: Display, B: Display> Args for (A, B) {
    fn formatted(&self) -> Vec<String> {
        vec![self.0.to_string(), self.1.to_string()]
    }
}

impl<A: Display, B: Display, C: Display> Args for (A, B, C) {
    fn formatted(&self) -> Vec<String> {
        vec![self.0.to_string(), self.1.to_string(), self.2.to_string()]
    }
}

impl<A: Display, B: Display, C: Display, D: Display> Args for (A, B, C, D) {
    fn formatted(&self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
        ]
    }
}

impl<A: Display, B: Display, C: Display, D: Display, E: Display> Args for (A, B, C, D, E) {
    fn formatted(&self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
            self.4.to_string(),
        ]
    }
}

fn format_message(message: &str, args: impl Args) -> String {
    let args = args.formatted();

    let mut inside_fmt = false;
    let pieces = message.split(|character| {
        if character == '{' {
            inside_fmt = true;

            true
        } else if character == '}' && inside_fmt {
            inside_fmt = false;

            true
        } else {
            inside_fmt
        }
    });

    pieces
        .enumerate()
        .map(|(index, string)| {
            if let Some(arg) = args.get(index) {
                format!("{string}{arg}")
            } else {
                string.to_string()
            }
        })
        .collect::<String>()
}

/* macro_rules! errors {
    ($( $error_name:ident: $error_code:literal => $error_message:expr );*;) => {
        #[derive(PartialEq, Clone, Copy, Debug)]
        pub enum ErrorCode {
            $(
                $error_name = $error_code
            ),*
        }

        impl ErrorCode {
            pub fn format(&self, args: impl Args) -> (ErrorCode, String) {
                match self {
                    $(
                        ErrorCode::$error_name => (*self, format_message($error_message, args))
                    ),*
                }
            }
        }
    };
}
 */

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ErrorCode {
    /// Caused while type-checking
    Type = 0x1250,
    /// Value not exists
    Reference = 0x1350,
    /// Required value has not been declared or user trying to access uninitialized variable
    Declaration = 0x1450,
    /// Caused for features that currently not implemented
    Implementing = 0x1950,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Specifier {
    pub property: String,
    pub alias: Option<String>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ImportClause {
    Named(String),
    NamedImports(Vec<Specifier>),
    NamespaceImport(String),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Import {
    pub clause: Option<ImportClause>,
    pub module: String,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ExportSource {
    CurrentModule,
    Module(String),
}

#[derive(PartialEq, Clone, Debug)]
pub enum ExportValue {
    Value(Value),
    Reference(String, Option<String>),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Export {
    pub source: ExportSource,
    pub value: ExportValue,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Module {
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct PropertySignature {
    pub name: String,
    pub nullable: bool,
    pub ty: Type,
}

#[derive(PartialEq, Clone, Debug)]
pub struct MethodSignature {
    pub name: Value,
    pub parameters: Vec<PropertySignature>,
    pub ty: Type,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ConstructSignature {
    pub parameters: Vec<PropertySignature>,
    pub ty: Type,
}

#[derive(PartialEq, Clone, Debug)]
pub struct CallSignature {
    pub parameters: Vec<PropertySignature>,
    pub ty: Type,
}

#[derive(PartialEq, Clone, Debug)]
pub struct IndexSignature {
    pub name: String,
    pub index_type: Type,
    pub ty: Type,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Signature {
    Property(PropertySignature),
    Call(CallSignature),
    Construct(ConstructSignature),
    Index(IndexSignature),
    Method(MethodSignature),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Public
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub nullable: bool,
    pub ty: Type,
    pub default: Option<Box<Value>>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ArrowParameter {
    pub name: String,
    pub nullable: bool,
    pub ty: Option<Type>,
    pub default: Option<Box<Value>>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ArrowFunction {
    pub is_async: bool,
    pub parameters: Vec<ArrowParameter>,
    pub ty: Option<Type>,
    pub body: Block,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    pub visibility: Visibility,
    pub overloads: Vec<Function>,
    pub is_async: bool,
    pub is_static: bool,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub ty: Type,
    pub body: Block,
}

use derivative::Derivative;
use std::sync::Arc;

#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct NativeFunction {
    pub visibility: Visibility,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub ty: Type,

    #[derivative(Debug = "ignore")]
    pub body: Arc<dyn Fn(&mut FArguments)>,
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &NativeFunction) -> bool {
        self.visibility == other.visibility
            && self.name == other.name
            && self.parameters == other.parameters
            && self.ty == other.ty
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Property {
    pub name: String,
    pub nullable: bool,
    pub ty: Type,
    pub init: Option<Box<Value>>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct EnumMember {
    pub name: String,
    pub init: Box<Value>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Field {
    pub prop: Property,
    pub value: Value,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Reference(Vec<String>, Scope),
    Array(Vec<Value>, ArraySize),
    Object(HashMap<Value, Value>),
    Number(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    ReturnValue(Box<Value>),
    ArrowFunction(ArrowFunction),
    NativeFunction(NativeFunction),
    Function(Function),
    TypeAlias {
        name: String,
        ty: Type,
    },
    Enum(Enum),
    Interface {
        name: String,
        extends: Vec<String>,
        signatures: Vec<Signature>,
    },
    Class {
        name: String,
        extends: Vec<String>,
        implements: Vec<String>,
        constructors: Vec<Function>,
        fields: Vec<Property>,
        methods: Vec<Function>,
    },
    ClassInstance(ClassInstance),
    Null,
    None,
    Error(Span, ErrorCode, String),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Enum {
    pub name: String,
    pub members: Vec<EnumMember>,
}

impl Enum {
    pub fn get_moved<N: AsRef<str>>(self, name: N) -> Option<EnumMember> {
        self.members
            .into_iter()
            .find(|member| member.name == name.as_ref())
    }

    pub fn get<N: AsRef<str>>(&self, name: N) -> Option<&EnumMember> {
        self.members
            .iter()
            .find(|member| member.name == name.as_ref())
    }

    pub fn get_mut<N: AsRef<str>>(&mut self, name: N) -> Option<&mut EnumMember> {
        self.members
            .iter_mut()
            .find(|member| member.name == name.as_ref())
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ClassInstance {
    pub name: String,
    pub fields: Vec<Field>,
}

impl ClassInstance {
    pub fn get_field_moved<N: AsRef<str>>(self, name: N) -> Option<Field> {
        self.fields
            .into_iter()
            .find(|field| field.prop.name == name.as_ref())
    }

    pub fn get_field<N: AsRef<str>>(&self, name: N) -> Option<&Field> {
        self.fields
            .iter()
            .find(|field| field.prop.name == name.as_ref())
    }

    pub fn get_field_mut<N: AsRef<str>>(&mut self, name: N) -> Option<&mut Field> {
        self.fields
            .iter_mut()
            .find(|field| field.prop.name == name.as_ref())
    }
}

impl Function {
    pub fn call(
        &self,
        span: Span,
        runtime: &mut Runtime,
        args: Vec<(Span, Value)>,
        lambda: Option<Block>,
    ) -> Value {
        let arg_count = self
            .parameters
            .iter()
            .filter(|parameter| parameter.default.is_none())
            .count();

        println!("{} < {arg_count}", args.len());

        if args.len() < arg_count {
            return Value::error(
                span,
                ErrorCode::Type,
                format!(
                    "wrong number of arguments: {} expected but {} given",
                    arg_count,
                    args.len()
                ),
            );
        }

        let zipped = self.parameters.clone().into_iter().zip(args);

        runtime.add_scope(format!("func:{}", self.name));

        for (argument, (span, value)) in zipped {
            if value.is_none() && lambda.is_some() {
                if let Type::FunctionType(_, params, ty) = argument.ty.clone() {
                    let value = span.wrap(Value::ArrowFunction(ArrowFunction {
                        is_async: false,
                        parameters: params
                            .into_iter()
                            .map(|param| ArrowParameter {
                                name: param.name.value.0,
                                nullable: param.nullable.value,
                                ty: Some(param.ty.value),
                                default: param
                                    .default
                                    .map(|default| Box::new(runtime.eval_expression(default))),
                            })
                            .collect(),
                        ty: Some(*ty),
                        body: lambda.unwrap(),
                    }));

                    runtime.set_variable(&argument.name, value);

                    break;
                }
            } else {
                if !value.is_type_of(&argument.ty) {
                    return Value::error(
                        span,
                        ErrorCode::Type,
                        format!(
                            "{} expected but {} given",
                            argument.ty,
                            value.value_type_of()
                        ),
                    );
                }

                runtime.set_variable(argument.name, span.wrap(value));
            }
        }

        let value = runtime.eval_code_block(self.body.clone());

        runtime.clear_scope_variables();
        runtime.remove_scope();

        value
    }
}

macro_rules! impl_value {
    ([$name:ident: $($type_name:ty)|*] => $callback:expr) => {
        $(
            impl From<$type_name> for Value {
                fn from($name: $type_name) -> Self {
                    $callback
                }
            }
        )*
    };
}

impl_value!([value: u8 | u16 | u32 | u64 | i8 | i16 | i32 | i64 | usize] => Value::Number(value as i64));

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl<K: Into<Value>, V: Into<Value>> From<HashMap<K, V>> for Value {
    fn from(value: HashMap<K, V>) -> Self {
        Value::Object(
            value
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(value: Vec<T>) -> Self {
        Value::Array(
            value.into_iter().map(|element| element.into()).collect(),
            ArraySize::Dynamic,
        )
    }
}

impl Value {
    pub fn error<T: Into<String>>(span: Span, code: ErrorCode, message: T) -> Self {
        Self::Error(span, code, message.into())
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            Value::ReturnValue(value) => value.is_primitive(),
            value => matches!(
                value,
                Value::Number(_)
                    | Value::Float(_)
                    | Value::Boolean(_)
                    | Value::String(_)
                    | Value::Null
                    | Value::None
            ),
        }
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(..))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let alternate = f.alternate();

        match self {
            Value::Array(elements, _) => {
                let mut list = &mut f.debug_list();

                for element in elements {
                    list = match alternate {
                        true => list.entry(&format_args!("{element:#}")),
                        false => list.entry(&format_args!("{element}")),
                    }
                }

                list.finish()
            }
            Value::Object(properties) => {
                let mut structure = &mut f.debug_struct("Object");

                for (name, value) in properties {
                    let name = match name {
                        Value::String(value) => value.clone(),
                        value => value.to_string(),
                    };

                    structure = match alternate {
                        true => structure.field(&name, &format_args!("{value:#}")),
                        false => structure.field(&name, &format_args!("{value}")),
                    }
                }

                structure.finish()
            }
            Value::Number(number) => number.fmt(f),
            Value::Float(float) => float.fmt(f),
            Value::Boolean(boolean) => boolean.fmt(f),
            Value::String(string) => write!(f, "{string:?}"),
            Value::ReturnValue(value) => value.fmt(f),
            Value::Function(function) => write!(
                f,
                "[Function {}({}) => {}]",
                function.name,
                function.parameters.len(),
                function.ty
            ),
            Value::Interface {
                name,
                extends: _,
                signatures,
            } => {
                let mut structure = &mut f.debug_struct(name);

                for signature in signatures {
                    structure = structure.field(
                        match signature {
                            Signature::Property(signature) => &signature.name,
                            Signature::Call(_) => name,
                            Signature::Construct(_) => "new",
                            Signature::Index(signature) => &signature.name,
                            Signature::Method(signature) => name,
                        },
                        signature,
                    );
                }

                structure.finish()
            }
            Value::Class {
                name,
                extends: _,
                implements: _,
                constructors,
                fields,
                methods,
            } => {
                let mut structure = &mut f.debug_struct(name);

                for constructor in constructors.clone() {
                    structure = structure.field("constructor", &Value::Function(constructor));
                }

                for field in fields {
                    structure = structure.field(&field.name, &field.init);
                }

                for method in methods {
                    structure = structure.field(&method.name, &Value::Function(method.clone()));
                }

                structure.finish()
            }
            Value::Null => write!(f, "null"),
            Value::None => write!(f, ""),
            Value::Error(span, code, message) => write!(
                f,
                "[{code:?}Exception]:{}:{}: {message}",
                span.line, span.line
            ),
            Value::ArrowFunction(function) => {
                write!(f, "[Function anonymous({})]", function.parameters.len())
            }
            Value::Enum(Enum { name, members }) => {
                let mut structure = &mut f.debug_struct(name);

                for member in members {
                    structure = structure.field(&member.name, &member.init);
                }

                structure.finish()
            }
            Value::TypeAlias { name, ty } => f.debug_tuple(name).field(ty).finish(),
            Value::NativeFunction(func) => write!(
                f,
                "[NativeFunction {}({})]",
                func.name,
                func.parameters.len()
            ),
            Value::ClassInstance(_) => todo!(),
            Value::Reference(reference, _) => write!(f, "[Reference({})]", reference.join(".")),
        }
    }
}

impl Value {
    pub fn format<T: Into<String>>(&self, file: T, source: T) -> String {
        let file: String = file.into();
        let source: String = source.into();

        match &self {
            Value::Error(span, code, message) => format!(
                "==> {red}{code:?}Exception{reset} at {green}{file}:{line}:{column}{reset}\n{gray}{pline:>3} |{reset}\n{gray}{line:>3} |{reset} {data}\n{gray}... |{reset}{nspace}{red}{length} {message}{reset}",
                pline = span.line - 1,
                line = span.line,
                column = span.column,
                length = "-".repeat(span.end - span.start),
                data = source.lines().collect::<Vec<_>>()[(span.line - 1) as usize],
                nspace = " ".repeat(span.column + 2 - (span.line.to_string().len())),
                gray = colors::css::DimGray::ANSI_FG,
                green = colors::css::LightSeaGreen::ANSI_FG,
                red = colors::css::IndianRed::ANSI_FG,
                reset = colors::Default::ANSI_FG
            ),
            _ => format!("{self}"),
        }
    }
}

impl Value {
    pub fn is_returned(&self) -> bool {
        matches!(self, Value::ReturnValue(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }

    pub fn is_type_of(&self, ty: &Type) -> bool {
        match ty {
            Type::UnionOrIntersectionOrPrimaryType(ty) => match ty {
                UnionOrIntersectionOrPrimaryType::UnionType(types) => types.iter().any(|ty| {
                    self.is_type_of(&Type::UnionOrIntersectionOrPrimaryType(
                        UnionOrIntersectionOrPrimaryType::IntersectionOrPrimaryType(ty.clone()),
                    ))
                }),
                UnionOrIntersectionOrPrimaryType::IntersectionOrPrimaryType(ty) => match ty {
                    IntersectionOrPrimaryType::IntersectionType(types) => types.iter().all(|ty| {
                        self.is_type_of(&Type::UnionOrIntersectionOrPrimaryType(
                            UnionOrIntersectionOrPrimaryType::IntersectionOrPrimaryType(
                                IntersectionOrPrimaryType::PrimaryType(ty.clone()),
                            ),
                        ))
                    }),
                    IntersectionOrPrimaryType::PrimaryType(ty) => match &ty {
                        PrimaryType::ParenthesizedType(ty) => self.is_type_of(ty),
                        PrimaryType::PredefinedType(ty) => match ty {
                            PredefinedType::Any => true,
                            PredefinedType::Number => matches!(self, Value::Number(_)),
                            PredefinedType::Float => matches!(self, Value::Float(_)),
                            PredefinedType::Boolean => matches!(self, Value::Boolean(_)),
                            PredefinedType::String => matches!(self, Value::String(_)),
                            PredefinedType::StringLiteral(literal) => match self {
                                Value::String(string) => string == literal,
                                _ => false,
                            },
                            PredefinedType::Symbol => true,
                            PredefinedType::Void => matches!(self, Value::None),
                            PredefinedType::Null => matches!(self, Value::Null),
                        },
                        PrimaryType::TypeReference(_, _) => true,
                        PrimaryType::ObjectType(members) => match self {
                            Value::Object(_fields) => true /* match members[0] {
                                TypeMember::PropertySignature(_) => true,
                                TypeMember::CallSignature(_) => true,
                                TypeMember::ConstructSignature(_) => true,
                                TypeMember::IndexSignature(_) => true,
                                TypeMember::MethodSignature(_) => true,
                            } */,
                            _ => false,
                        },
                        PrimaryType::ArrayType(ty, size) => match self {
                            Value::Array(elements, _) => match size {
                                ArraySize::Fixed(size) => {
                                    elements.len() == *size
                                        && elements.iter().all(|element| {
                                            element.is_type_of(&(*ty.clone()).into())
                                        })
                                }
                                ArraySize::Dynamic => elements
                                    .iter()
                                    .all(|element| element.is_type_of(&(*ty.clone()).into())),
                            },
                            _ => false,
                        },
                        PrimaryType::TupleType(types) => match self {
                            Value::Array(elements, _) => {
                                elements.len() == types.len()
                                    && elements
                                        .iter()
                                        .zip(types)
                                        .all(|(element, ty)| element.is_type_of(ty))
                            }
                            _ => false,
                        },
                        PrimaryType::TypeQuery => true,
                        PrimaryType::ThisType => true,
                    },
                },
            },
            Type::FunctionType(_generics, params, ty) => match self {
                Value::Function(_) => todo!(),
                Value::ArrowFunction(func) => {
                    func.parameters.len() == params.len()
                        && params.iter().enumerate().all(|(index, param)| {
                            match func.parameters[index].ty.as_ref() {
                                Some(ty) => ty == &param.ty.value,
                                None => true,
                            }
                        })
                        && (func.ty.is_none()
                            || func
                                .ty
                                .as_ref()
                                .is_some_and(|func_ty| func_ty == ty.as_ref()))
                }
                _ => false,
            },
            Type::ConstructorType(_, _, _) => match self {
                Value::Function(_) => todo!(),
                _ => false,
            },
        }
    }

    pub fn value_type_of(&self) -> Type {
        match self {
            Value::Array(elements, size) => {
                let elements = elements
                    .iter()
                    .map(|element| element.value_type_of())
                    .collect::<Vec<_>>();

                let ty = if elements.iter().all(|ty| ty == &elements[0]) {
                    elements
                        .get(0)
                        .cloned()
                        .unwrap_or(PredefinedType::Any.into())
                } else {
                    panic!("all elements in array must be one type");
                };

                if let Type::UnionOrIntersectionOrPrimaryType(
                    UnionOrIntersectionOrPrimaryType::IntersectionOrPrimaryType(
                        IntersectionOrPrimaryType::PrimaryType(ty),
                    ),
                ) = ty
                {
                    PrimaryType::ArrayType(Box::new(ty), *size).into()
                } else {
                    panic!("can't infer type");
                }
            }
            Value::Object(_) => todo!(),
            Value::Number(_) => PredefinedType::Number.into(),
            Value::Float(_) => PredefinedType::Float.into(),
            Value::Boolean(_) => PredefinedType::Boolean.into(),
            Value::String(_) => PredefinedType::String.into(),
            Value::ReturnValue(value) => value.value_type_of(),
            Value::ArrowFunction(func) => PredefinedType::Any.into(),
            Value::Function(_) => todo!(),
            Value::Interface {
                name,
                extends,
                signatures,
            } => todo!(),
            Value::Class {
                name,
                extends,
                implements,
                constructors,
                fields,
                methods,
            } => todo!(),
            Value::Null => PredefinedType::Null.into(),
            Value::None => PredefinedType::Void.into(),
            Value::Error(span, code, message) => PredefinedType::Void.into(),
            Value::Enum(_) => todo!(),
            Value::TypeAlias { name, ty } => todo!(),
            Value::NativeFunction(_) => todo!(),
            Value::ClassInstance(_) => PrimaryType::ThisType.into(),
            Value::Reference(..) => todo!(),
        }
    }

    pub fn type_of(&self) -> &str {
        match self {
            Value::Array(..) => "array",
            Value::Object(_) => "object",
            Value::Number(_) => "number",
            Value::Float(_) => "float",
            Value::Boolean(_) => "boolean",
            Value::String(_) => "string",
            Value::ReturnValue(value) => value.type_of(),
            Value::Function(_) => "Function",
            Value::Interface { name, .. } => name,
            Value::Class { name, .. } => name,
            Value::Null => "null",
            Value::None => "",
            Value::Error { .. } => "Exception",
            Value::ArrowFunction(_) => "LinearFunction",
            Value::Enum(Enum { name, .. }) => name,
            Value::TypeAlias { name, .. } => name,
            Value::NativeFunction(_) => todo!(),
            Value::ClassInstance(_) => todo!(),
            Value::Reference(..) => todo!(),
        }
    }
}

impl Eq for Value {}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Number(number) => number.hash(state),
            Value::Boolean(boolean) => boolean.hash(state),
            Value::String(string) => string.hash(state),
            _ => "".hash(state),
        }
    }
}

impl From<Literal> for Value {
    fn from(literal: Literal) -> Self {
        match literal {
            Literal::String(string) => Value::String(string.value),
            Literal::Number(number) => Value::Number(number.value),
            Literal::Float(float) => Value::Float(float.value),
            Literal::Boolean(boolean) => Value::Boolean(boolean.value),
        }
    }
}
