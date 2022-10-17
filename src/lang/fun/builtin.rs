use std::collections::BTreeMap;
use crate::lang::fun::{FunRef, Fun};
use crate::lang::fun::builtin::munge_for_metastaar::MungeForMetastaar;

pub(crate) mod munge_for_metastaar;
pub(crate) mod cow_say;

pub(crate) trait Gen where Self: Fun {
    fn new() -> Self;
}

pub(crate) fn get_fun_ref<G: Gen>(name: &str) -> FunRef {
    G::new().into_fun_ref(name.to_string())
}

pub(crate) fn get_builtins() -> Vec<FunRef> {
    let munge_for_metastaar = get_fun_ref::<MungeForMetastaar>("munge_for_metastaar");
    vec![munge_for_metastaar]
}

fn add_fun(funs: &mut BTreeMap<String, FunRef>, fun_ref: FunRef) {
    funs.insert(fun_ref.name.clone(), fun_ref);
}

pub(crate) fn get_builtin_funs() -> BTreeMap<String, FunRef> {
    let mut funs: BTreeMap<String, FunRef> = BTreeMap::new();
    for fun_ref in get_builtins() {
        add_fun(&mut funs, fun_ref);
    }
    funs
}