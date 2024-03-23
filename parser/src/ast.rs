use core::fmt;

use tsr_lexer::globals::Positioned;
use tsr_lexer::token::Modifier;
use tsr_lexer::token::Operator;

#[derive(PartialEq, Debug, Clone)]
pub enum Infix {
    Assign,
    Increment,
    Plus,
    Decrement,
    Minus,
    Divide,
    Multiply,
    Inverse,
    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,
}

#[derive(PartialEq, Hash, Debug, Eq, Clone)]
pub struct Ident(pub String);

impl Ident {
    pub fn new<T: Into<String>>(string: T) -> Ident {
        Ident(string.into())
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    PLowest,
    PEquals,
    PLessGreater,
    PSum,
    PProduct,
    PCall,
    PIndex,
}

pub type Block = Positioned<Vec<Positioned<Statement>>>;

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    ImportDeclaration(Box<Positioned<ImportDeclaration>>),
    TypeAliasDeclaration(Positioned<TypeAliasDeclaration>),
    InterfaceDeclaration(Positioned<InterfaceDeclaration>),
    FunctionDeclaration(Positioned<FunctionDeclaration>),
    EnumDeclaration(Positioned<EnumDeclaration>),
    ExportDeclaration(Positioned<ExportDeclaration>),
    ClassDeclaration(Positioned<ClassDeclaration>),
    VariableStatement(Positioned<VariableStatement>),
    IfStatement(Box<Positioned<IfStatement>>),
    ReturnStatement(Positioned<Expression>),
    Expression(Positioned<Expression>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ExportDeclaration {
    Default(Positioned<ExportDefaultElement>),
    Single(Positioned<ExportSingleElement>),
    List(Positioned<ExportListElement>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ExportDefaultElement {
    FunctionDeclaration(Positioned<FunctionDeclaration>),
    ClassDeclaration(Positioned<ClassDeclaration>),
    Expression(Positioned<Expression>),
    IdentifierReference(Positioned<Ident>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ExportSingleElement {
    VariableStatement(Positioned<VariableStatement>),
    FunctionDeclaration(Positioned<FunctionDeclaration>),
    ClassDeclaration(Positioned<ClassDeclaration>),
    InterfaceDeclaration(Positioned<InterfaceDeclaration>),
    TypeAliasDeclaration(Positioned<TypeAliasDeclaration>),
    EnumDeclaration(Positioned<EnumDeclaration>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct ClassDeclaration {
    pub name: Positioned<Ident>,
    pub type_parameters: Vec<Positioned<TypeParameter>>,
    pub extends: Vec<Positioned<Ident>>,
    pub implements: Vec<Positioned<Ident>>,
    pub body: Vec<Positioned<ClassElement>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ClassElement {
    ConstructorDeclaration(Positioned<ConstructorDeclaration>),
    PropertyMemberDeclaration(Positioned<PropertyMemberDeclaration>),
    IndexMemberDeclaration(Positioned<IndexSignature>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct ConstructorDeclaration {
    pub modifiers: Vec<Positioned<Modifier>>,
    pub parameters: Vec<Positioned<Parameter>>,
    pub body: Block,
}

#[derive(PartialEq, Debug, Clone)]
pub enum PropertyMemberDeclaration {
    MemberVariableDeclaration(Positioned<MemberVariableDeclaration>),
    MemberFunctionDeclaration(Positioned<MemberFunctionDeclaration>),
    MemberAccessorDeclaration(Positioned<MemberAccessorDeclaration>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct MemberVariableDeclaration {
    pub modifiers: Vec<Positioned<Modifier>>,
    pub name: Positioned<PropertyName>,
    // pub optional: Positioned<bool>,
    pub ty: Option<Positioned<Type>>,
    pub initializer: Option<Positioned<Expression>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct MemberFunctionDeclaration {
    pub modifiers: Vec<Positioned<Modifier>>,
    pub name: Positioned<PropertyName>,
    pub type_parameters: Vec<Positioned<TypeParameter>>,
    pub parameters: Vec<Positioned<Parameter>>,
    pub ty: Positioned<Type>,
    pub body: Block,
}

#[derive(PartialEq, Debug, Clone)]
pub struct MemberAccessorDeclaration {
    pub modifiers: Vec<Positioned<Modifier>>,
    pub kind: Positioned<AccessorKind>,
    pub name: Positioned<PropertyName>,
    pub parameter: Option<Positioned<Ident>>,
    pub ty: Positioned<Type>,
    pub body: Block,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AccessorKind {
    Getter,
    Setter,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ExportListElement {
    Namespace(Positioned<String>),
    NamedExports(Positioned<Vec<Positioned<ExportSpecifier>>>),
    NamespaceExports(
        Positioned<Vec<Positioned<ExportSpecifier>>>,
        Positioned<String>,
    ),
}

#[derive(PartialEq, Debug, Clone)]
pub struct ExportSpecifier {
    pub property_name: Option<Positioned<Ident>>,
    pub name: Positioned<Ident>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Parameter {
    pub name: Positioned<Ident>,
    pub nullable: Positioned<bool>,
    pub ty: Positioned<Type>,
    pub default: Option<Positioned<Expression>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ArrowParameter {
    pub name: Positioned<Ident>,
    pub nullable: Positioned<bool>,
    pub ty: Option<Positioned<Type>>,
    pub default: Option<Positioned<Expression>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: Positioned<Ident>,
    pub modifiers: Vec<Positioned<Modifier>>,
    pub type_parameters: Vec<Positioned<TypeParameter>>,
    pub parameters: Vec<Positioned<Parameter>>,
    pub ty: Positioned<Type>,
    pub body: Option<Block>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct EnumDeclaration {
    pub name: Positioned<Ident>,
    pub members: Vec<Positioned<EnumMember>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct EnumMember {
    pub name: Positioned<Ident>,
    pub initializer: Option<Positioned<Expression>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct InterfaceDeclaration {
    pub type_parameters: Vec<Positioned<TypeParameter>>,
    pub name: Positioned<Ident>,
    pub members: Vec<Positioned<TypeMember>>,
    pub extends: Vec<Positioned<Ident>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct IfStatement {
    pub expression: Positioned<Expression>,
    pub then_statement: Positioned<Statement>,
    pub else_statement: Option<Positioned<Statement>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ImportDeclaration {
    pub import_clause: Option<Positioned<ImportClause>>,
    pub module_specifier: Positioned<Literal>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ImportClause {
    Named(Positioned<Ident>),
    NamedImports(Vec<Positioned<ImportSpecifier>>),
    NamespaceImport(Positioned<Ident>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct ImportSpecifier {
    pub is_type_only: Positioned<bool>,
    pub property_name: Option<Positioned<Ident>>,
    pub name: Positioned<Ident>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ArraySize {
    Fixed(usize),
    Dynamic,
}

#[derive(PartialEq, Debug, Clone)]
pub struct CallSignature(
    pub Vec<Positioned<TypeParameter>>,
    pub Vec<Positioned<Parameter>>,
    pub Positioned<Type>,
);

#[derive(PartialEq, Debug, Clone)]
pub struct ConstructSignature(
    pub Vec<Positioned<TypeParameter>>,
    pub Vec<Positioned<Parameter>>,
    pub Positioned<Type>,
);

#[derive(PartialEq, Debug, Clone)]
pub struct IndexSignature(
    pub Positioned<Ident>,
    pub Positioned<Type>,
    pub Positioned<Type>,
);

#[derive(PartialEq, Debug, Clone)]
pub struct MethodSignature(
    pub Positioned<PropertyName>,
    pub Positioned<bool>,
    pub Box<Positioned<CallSignature>>,
);

#[derive(PartialEq, Debug, Clone)]
pub enum PropertyName {
    LiteralPropertyName(Positioned<Literal>),
    ComputedPropertyName(Positioned<Expression>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum TypeMember {
    PropertySignature(Positioned<PropertySignature>),
    CallSignature(Positioned<CallSignature>),
    ConstructSignature(Positioned<ConstructSignature>),
    IndexSignature(Positioned<IndexSignature>),
    MethodSignature(Positioned<MethodSignature>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    UnionOrIntersectionOrPrimaryType(UnionOrIntersectionOrPrimaryType),
    FunctionType(Vec<TypeParameter>, Vec<Parameter>, Box<Type>),
    ConstructorType(Vec<TypeParameter>, Vec<Parameter>, Box<Type>),
}

impl Default for Type {
    fn default() -> Self {
        PredefinedType::Void.into()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum UnionOrIntersectionOrPrimaryType {
    UnionType(Vec<IntersectionOrPrimaryType>),
    IntersectionOrPrimaryType(IntersectionOrPrimaryType),
}

#[derive(PartialEq, Debug, Clone)]
pub enum IntersectionOrPrimaryType {
    IntersectionType(Vec<PrimaryType>),
    PrimaryType(PrimaryType),
}

#[derive(PartialEq, Debug, Clone)]
pub enum PrimaryType {
    ParenthesizedType(Box<Type>),
    PredefinedType(PredefinedType),
    TypeReference(Ident, Vec<Ident>),
    ObjectType(Vec<TypeMember>),
    ArrayType(Box<PrimaryType>, ArraySize),
    TupleType(Vec<Type>),
    TypeQuery,
    ThisType,
}

#[derive(PartialEq, Debug, Clone)]
pub enum PredefinedType {
    Any,
    Number,
    Float,
    Boolean,
    String,
    StringLiteral(String),
    Symbol,
    Null,
    Void,
}

impl From<PredefinedType> for PrimaryType {
    fn from(value: PredefinedType) -> Self {
        PrimaryType::PredefinedType(value)
    }
}

impl From<PredefinedType> for Type {
    fn from(value: PredefinedType) -> Self {
        Self::UnionOrIntersectionOrPrimaryType(
            UnionOrIntersectionOrPrimaryType::IntersectionOrPrimaryType(
                IntersectionOrPrimaryType::PrimaryType(PrimaryType::PredefinedType(value)),
            ),
        )
    }
}

impl From<PrimaryType> for Type {
    fn from(value: PrimaryType) -> Self {
        Self::UnionOrIntersectionOrPrimaryType(
            UnionOrIntersectionOrPrimaryType::IntersectionOrPrimaryType(
                IntersectionOrPrimaryType::PrimaryType(value),
            ),
        )
    }
}

impl From<Vec<PrimaryType>> for Type {
    fn from(value: Vec<PrimaryType>) -> Self {
        Self::UnionOrIntersectionOrPrimaryType(
            UnionOrIntersectionOrPrimaryType::IntersectionOrPrimaryType(
                IntersectionOrPrimaryType::IntersectionType(value),
            ),
        )
    }
}

impl fmt::Display for PredefinedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PredefinedType::Any => write!(f, "any"),
            PredefinedType::Number => write!(f, "number"),
            PredefinedType::Float => write!(f, "float"),
            PredefinedType::Boolean => write!(f, "boolean"),
            PredefinedType::String => write!(f, "string"),
            PredefinedType::StringLiteral(literal) => write!(f, "\"{literal}\""),
            PredefinedType::Symbol => write!(f, "symbol"),
            PredefinedType::Null => write!(f, "null"),
            PredefinedType::Void => write!(f, "void"),
        }
    }
}

impl fmt::Display for PrimaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimaryType::ParenthesizedType(ty) => write!(f, "({})", **ty),
            PrimaryType::PredefinedType(ty) => ty.fmt(f),
            PrimaryType::TypeReference(name, generics) => {
                let result = write!(f, "{name}");

                if !generics.is_empty() {
                    write!(f, "<")?;

                    let last = generics.last();

                    for generic in generics {
                        generic.fmt(f)?;

                        if !last.is_some_and(|g| g == generic) {
                            write!(f, ", ")?;
                        }
                    }

                    write!(f, ">")?;
                }

                result
            }
            PrimaryType::ObjectType(properties) => {
                write!(f, "{{ ")?;

                let last = properties.last();

                for member in properties {
                    match member {
                        TypeMember::PropertySignature(signature) => {
                            let PropertySignature {
                                modifiers,
                                name,
                                nullable,
                                ty,
                            } = &signature.value;

                            let modifiers = modifiers
                                .iter()
                                .map(|m| match m.value {
                                    Modifier::Public => "public",
                                    Modifier::Private => "private",
                                    Modifier::Protected => "protected",
                                    Modifier::Async => "async",
                                    Modifier::Static => "static",
                                })
                                .collect::<Vec<_>>();
                            let modifiers = if !modifiers.is_empty() {
                                format!("{} ", modifiers.join(", "))
                            } else {
                                "".into()
                            };

                            let name = &name.value;
                            let nullable = match nullable.value {
                                true => "?",
                                false => "",
                            };
                            let ty = &ty.value;

                            write!(f, "{modifiers}{name}{nullable}: {ty}")?;
                        }
                        TypeMember::CallSignature(_) => todo!(),
                        TypeMember::ConstructSignature(_) => todo!(),
                        TypeMember::IndexSignature(signature) => {
                            let signature = &signature.value;

                            write!(
                                f,
                                "[{}: {}]: {}",
                                signature.0.value, signature.1.value, signature.2.value
                            )?;
                        }
                        TypeMember::MethodSignature(_) => todo!(),
                    }

                    if !last.is_some_and(|p| p == member) {
                        write!(f, ", ")?;
                    }
                }

                write!(f, " }}")
            }
            PrimaryType::ArrayType(ty, size) => {
                ty.fmt(f)?;

                write!(f, "[")?;

                let size = match size {
                    ArraySize::Fixed(size) => size.to_string(),
                    ArraySize::Dynamic => "".into(),
                };

                write!(f, "{size}")?;

                write!(f, "]")
            }
            PrimaryType::TupleType(tuple) => {
                write!(f, "[")?;

                let last = tuple.last();

                for ty in tuple {
                    ty.fmt(f)?;

                    if !last.is_some_and(|t| t == ty) {
                        write!(f, ", ")?;
                    }
                }

                write!(f, "]")
            }
            PrimaryType::TypeQuery => write!(f, "unknown"),
            PrimaryType::ThisType => write!(f, "this"),
        }
    }
}

impl fmt::Display for IntersectionOrPrimaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntersectionOrPrimaryType::IntersectionType(types) => {
                let last = types.last();

                for ty in types {
                    ty.fmt(f)?;

                    if !last.is_some_and(|t| t == ty) {
                        write!(f, " & ")?;
                    }
                }

                Ok(())
            }
            IntersectionOrPrimaryType::PrimaryType(ty) => ty.fmt(f),
        }
    }
}

impl fmt::Display for TypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = write!(f, "{}", self.name.value);

        if let Some(constraint) = &self.constraint {
            return write!(f, " extends {}", constraint.value);
        }

        if let Some(default) = &self.default {
            return write!(f, " = {}", default.value);
        }

        result
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.value)?;

        if self.nullable.value {
            write!(f, "?")?;
        }

        let result = write!(f, ": {}", self.ty.value);

        if let Some(default) = &self.default {
            return write!(f, " = {:?}", default.value);
        }

        result
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::UnionOrIntersectionOrPrimaryType(ty) => match ty {
                UnionOrIntersectionOrPrimaryType::UnionType(types) => {
                    let last = types.last();

                    for ty in types {
                        ty.fmt(f)?;

                        if !last.is_some_and(|t| t == ty) {
                            write!(f, " | ")?;
                        }
                    }

                    Ok(())
                }
                UnionOrIntersectionOrPrimaryType::IntersectionOrPrimaryType(ty) => ty.fmt(f),
            },
            Type::FunctionType(generics, parameters, ty) => {
                if !generics.is_empty() {
                    write!(f, "<")?;

                    let last = generics.last();

                    for generic in generics {
                        generic.fmt(f)?;

                        if !last.is_some_and(|g| g == generic) {
                            write!(f, ", ")?;
                        }
                    }

                    write!(f, ">")?;
                }

                write!(f, "(")?;

                if !parameters.is_empty() {
                    let last = parameters.last();

                    for parameter in parameters {
                        parameter.fmt(f)?;

                        if !last.is_some_and(|p| p == parameter) {
                            write!(f, ", ")?;
                        }
                    }
                }

                write!(f, ") => {ty}")
            }
            Type::ConstructorType(generics, parameters, ty) => {
                write!(f, "new")?;

                if !generics.is_empty() {
                    write!(f, "<")?;

                    let last = generics.last();

                    for generic in generics {
                        generic.fmt(f)?;

                        if !last.is_some_and(|g| g == generic) {
                            write!(f, ", ")?;
                        }
                    }

                    write!(f, ">")?;
                }

                write!(f, "(")?;

                if !parameters.is_empty() {
                    let last = parameters.last();

                    for parameter in parameters {
                        parameter.fmt(f)?;

                        if !last.is_some_and(|p| p == parameter) {
                            write!(f, ", ")?;
                        }
                    }
                }

                write!(f, "): {ty}")
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PropertySignature {
    pub modifiers: Vec<Positioned<Modifier>>,
    pub name: Positioned<Ident>,
    pub nullable: Positioned<bool>,
    pub ty: Positioned<Type>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TypeParameter {
    pub name: Positioned<Ident>,
    pub constraint: Option<Positioned<Type>>,
    pub default: Option<Positioned<Type>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TypeAliasDeclaration {
    pub name: Positioned<Ident>,
    pub type_parameters: Vec<Positioned<TypeParameter>>,
    pub ty: Positioned<Type>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct VariableStatement {
    pub mutable: Positioned<bool>,
    pub declarations: Vec<Positioned<VariableDeclaration>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct VariableDeclaration {
    pub name: Positioned<Ident>,
    pub ty: Option<Positioned<Type>>,
    pub nullable: Positioned<bool>,
    pub initializer: Option<Positioned<Expression>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct SwitchVariant {
    pub value: Positioned<Expression>,
    pub callback: Positioned<Statement>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct MatchExpression {
    pub target: Positioned<Expression>,
    pub variants: Vec<Positioned<SwitchVariant>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionCallExpression {
    pub function: Box<Positioned<Expression>>,
    pub arguments: Vec<Positioned<Expression>>,
    pub lambda: Option<Block>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct NewExpression {
    pub expression: Box<Positioned<Expression>>,
    pub type_parameters: Vec<Positioned<TypeParameter>>,
    pub arguments: Vec<Positioned<Expression>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    BinaryExpression(Box<Positioned<BinaryExpression>>),
    IndexExpression(Box<Positioned<IndexExpression>>),
    MatchExpression(Box<Positioned<MatchExpression>>),
    FunctionCallExpression(Box<Positioned<FunctionCallExpression>>),
    NewExpression(Positioned<NewExpression>),
    Block(Block),
    Literal(Positioned<Literal>),
    Ident(Positioned<Ident>),
    Array {
        elements: Vec<Positioned<Expression>>,
        is_dynamic: Positioned<bool>,
    },
    ArrowFunction(Box<Positioned<ArrowFunction>>),
    This,
    Null,
}

#[derive(PartialEq, Debug, Clone)]
pub struct IndexExpression {
    pub target: Positioned<Expression>,
    pub index: Positioned<Expression>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ArrowFunction {
    pub modifiers: Vec<Positioned<Modifier>>,
    pub type_parameters: Vec<Positioned<TypeParameter>>,
    pub parameters: Vec<Positioned<ArrowParameter>>,
    pub ty: Option<Positioned<Type>>,
    pub body: Positioned<Expression>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    String(Positioned<String>),
    Number(Positioned<i64>),
    Float(Positioned<f64>),
    Boolean(Positioned<bool>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct BinaryExpression {
    pub left: Positioned<Expression>,
    pub operator: Positioned<Operator>,
    pub right: Positioned<Expression>,
}
