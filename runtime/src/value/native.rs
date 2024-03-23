use super::Value;

pub trait NativeObject {
    fn build_object(&'static self) -> Value;
}

pub trait NativeModule {
    fn build_module(&'static self) -> Module;
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub(crate) exports: Vec<(String, Value)>,
}

impl Module {
    pub fn new<N: AsRef<str>>(name: N) -> Self {
        let name: &str = name.as_ref();

        Self {
            name: name.into(),
            exports: Default::default(),
        }
    }

    pub fn export<N: AsRef<str>>(&mut self, name: N, value: Value) {
        let name: &str = name.as_ref();

        self.exports.push((name.into(), value))
    }
}

