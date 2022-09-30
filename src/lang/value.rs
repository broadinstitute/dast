use std::fmt::{Display, Formatter};

pub enum Value {
    Unit
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "()")
    }
}
