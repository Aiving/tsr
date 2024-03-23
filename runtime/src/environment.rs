use std::cmp::Reverse;
use std::iter::Peekable;
use std::sync::Arc;
use std::sync::Mutex;
use std::vec::IntoIter;

use tsr_lexer::util::VecExt;

use super::value::Value;

pub type Context = Arc<Mutex<Environment>>;
pub type Scope = Vec<String>;

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: String,
    pub scope: Scope,
    pub value: Value,
}

#[derive(Default, Clone, Debug)]
pub struct Environment {
    store: Vec<Variable>,
}

unsafe impl Send for Environment {}
unsafe impl Sync for Environment {}

impl Environment {
    pub fn new() -> Context {
        Arc::new(Mutex::new(Self::default()))
    }

    pub fn extend(&mut self, context: Context) {
        self.store.extend(context.lock().unwrap().store.clone())
    }

    fn modify(mut path: Peekable<IntoIter<&str>>, current_value: &mut Value, value: Value) {
        if let Some(current) = path.next() {
            let next = path.peek();

            match current_value {
                Value::Array(_elements, _) => {}
                Value::Object(properties) => {
                    let current = Value::String(current.into());

                    if next.is_some() {
                        if let Some(property_value) = properties.get_mut(&current) {
                            Self::modify(path, property_value, value)
                        }
                    } else {
                        properties.insert(current, value);
                    }
                }
                Value::ClassInstance(instance) => {
                    match (next.is_some(), instance.get_field_mut(current)) {
                        (true, Some(field)) => Self::modify(path, &mut field.value, value),
                        (false, Some(field)) => {
                            field.value = value;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    pub fn set<N: AsRef<str>>(&mut self, name: &[N], scope: Scope, value: Value) {
        match name.len() {
            0 => println!("warning: tried to set value for nothing"),
            1 => {
                let name = name[0].as_ref();

                if let Some(variable) = self.get_mut(name, scope.clone()) {
                    variable.value = value;
                } else {
                    self.store.push(Variable {
                        name: name.into(),
                        scope,
                        value,
                    });
                }
            }
            _ => {
                if let Some(variable) = self.get_mut(name[0].as_ref(), scope) {
                    let path = name[1..]
                        .iter()
                        .map(|item| item.as_ref())
                        .collect::<Vec<_>>()
                        .into_iter()
                        .peekable();

                    Self::modify(path, &mut variable.value, value)
                }
            }
        }
    }

    pub fn exists<N: AsRef<str>>(&self, name: N, scope: Scope) -> bool {
        let name: &str = name.as_ref();

        self.store
            .includes(|variable| variable.name == name && variable.scope == scope)
    }

    pub fn get_mut<N: AsRef<str>>(&mut self, name: N, scope: Scope) -> Option<&mut Variable> {
        let mut scopes = scope
            .into_iter()
            .fold::<Vec<Scope>, _>(vec![], |mut acc, e| {
                let mut arr = acc.last().cloned().unwrap_or_default();

                arr.push(e);
                acc.push(arr);

                acc
            });

        scopes.sort_by_key(|scope| Reverse(scope.len()));

        let mut index = None;

        for scope in scopes {
            let name = name.as_ref();

            let position = self
                .store
                .iter()
                .position(|variable| variable.name == name && variable.scope == scope);

            if position.is_some() {
                index = position;
            }
        }

        if let Some(index) = index {
            self.store.get_mut(index)
        } else {
            None
        }
    }

    pub fn remove_by_scope(&mut self, scope: Scope) {
        self.store.retain(|variable| variable.scope != scope);
    }

    pub fn remove_by_scope_filtered(&mut self, scope: Scope, variables: &[String]) {
        self.store
            .retain(|variable| variable.scope != scope || variables.contains(&variable.name));
    }

    pub fn remove<N: AsRef<str>>(&mut self, name: N, scope: Scope) {
        let name = name.as_ref();

        self.store
            .retain(|variable| variable.name != name && variable.scope != scope);
    }

    pub fn get<N: AsRef<str>>(&self, name: N, scope: Scope) -> Option<&Variable> {
        let mut scopes = scope
            .into_iter()
            .fold::<Vec<Scope>, _>(vec![], |mut acc, e| {
                let mut arr = acc.last().cloned().unwrap_or_default();

                arr.push(e);
                acc.push(arr);

                acc
            });

        scopes.sort_by_key(|scope| Reverse(scope.len()));

        for scope in scopes {
            let name = name.as_ref();

            if let Some(variable) = self
                .store
                .iter()
                .find(|variable| variable.name == name && variable.scope == scope)
            {
                return Some(variable);
            }
        }

        None
    }
}
