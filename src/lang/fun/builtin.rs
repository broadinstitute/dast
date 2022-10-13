use std::collections::BTreeMap;
use crate::lang::fun::Fun;

pub(crate) mod munge_for_metastaar;

fn add_fun(funs: &mut BTreeMap<String, Fun>, fun: Fun) {
    funs.insert(fun.name.clone(), fun);
}

pub(crate) fn get_builtin_funs() -> BTreeMap<String, Fun> {
    let mut funs: BTreeMap<String, Fun> = BTreeMap::new();
    add_fun(&mut funs, munge_for_metastaar::MungeForMetastaar::new_fun());
    funs
}