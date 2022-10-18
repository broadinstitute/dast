use std::fmt::{Display, Formatter};

pub enum Value {
    Unit,
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Unit => { writeln!(f, "()") }
            Value::String(string) => { writeln!(f, "{}", string) }
        }
    }
}
