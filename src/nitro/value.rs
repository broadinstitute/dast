use std::fmt::{Display, Formatter};

pub(crate) enum Value {
    Unit
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "()")
    }
}
