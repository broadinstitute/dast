use std::collections::BTreeMap;
use crate::lang::fun::{FunRef, Fun};

pub(crate) mod munge_for_metastaar;
pub(crate) mod cow_say;

fn add_fun(funs: &mut BTreeMap<String, FunRef>, fun: FunRef) {
    funs.insert(fun.name.clone(), fun);
}

pub(crate) fn get_builtin_funs() -> BTreeMap<String, FunRef> {
    let mut funs: BTreeMap<String, FunRef> = BTreeMap::new();
    add_fun(&mut funs, munge_for_metastaar::MungeForMetastaar::new().into_fun("munge_for_metastaar".to_string()));
    funs
}