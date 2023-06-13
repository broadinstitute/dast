use std::collections::BTreeMap;
use crate::lang::fun::{FunRef, Fun};
use crate::lang::fun::builtin::col_covs::ColCovs;
use crate::lang::fun::builtin::wisdom::Wisdom;
use crate::lang::fun::builtin::munge_for_metastaar::MungeForMetastaar;
use crate::lang::fun::builtin::phenet::Phenet;
use crate::lang::fun::builtin::quit::Quit;
use crate::lang::fun::builtin::subset_wilcox::SubsetWilcox;

mod munge_for_metastaar;
mod wisdom;
mod quit;
mod group;
mod phenet;
mod subset_wilcox;
mod col_covs;

pub(crate) trait Gen where Self: Fun {
    fn new() -> Self;
}

pub(crate) fn get_fun_ref<G: 'static + Gen>(name: &str) -> FunRef {
    G::new().into_fun_ref(name.to_string())
}

pub(crate) fn get_builtins() -> Vec<FunRef> {
    let munge_for_metastaar = get_fun_ref::<MungeForMetastaar>("munge_for_metastaar");
    let fortune = get_fun_ref::<Wisdom>("wisdom");
    let quit = get_fun_ref::<Quit>("quit");
    let phenet = get_fun_ref::<Phenet>("phenet");
    let subset_wilcox = get_fun_ref::<SubsetWilcox>("subset_wilcox");
    let col_covs = get_fun_ref::<ColCovs>("col_covs");
    vec![munge_for_metastaar, fortune, quit, phenet, subset_wilcox, col_covs]
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