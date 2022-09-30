use jati::trees::types::Type;
use jati::engine::var::Var as JatiVar;

pub(crate) struct Var {
    tpe: Type,
}

impl JatiVar for Var {
    fn tpe(&self) -> Type { self.tpe }
}
