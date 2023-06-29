use std::fmt::{Display, Formatter};

pub enum Value {
    Unit,
    String(String),
}

impl Value {
    pub fn is_unit(&self) -> bool {
        match self {
            Value::Unit => { true }
            Value::String(_) => { false }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Unit => { writeln!(f, "()") }
            Value::String(string) => { writeln!(f, "{}", string) }
        }
    }
}
