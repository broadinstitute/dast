use std::collections::BTreeMap;
use crate::lang::fun::{FunRef, Fun};
use crate::lang::fun::builtin::add_quotient::AddQuotient;
use crate::lang::fun::builtin::covs::Covs;
use crate::lang::fun::builtin::json_to_tsv::JsonToTsv;
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
mod covs;
mod add_quotient;
mod json_to_tsv;

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
    let covs = get_fun_ref::<Covs>("covs");
    let add_quotient = get_fun_ref::<AddQuotient>("add_quotient");
    let json_to_tsv = get_fun_ref::<JsonToTsv>("json_to_tsv");
    vec![munge_for_metastaar, fortune, quit, phenet, subset_wilcox, covs, add_quotient, json_to_tsv]
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