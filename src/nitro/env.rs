use std::collections::BTreeMap;

pub(crate) struct Env {
    pub(crate) args: BTreeMap<String, Vec<String>>
}

impl Env {
    fn ensure_args_key(args: &mut BTreeMap<String, Vec<String>>, key: &str) {
        if !args.contains_key(key) {
            args.insert(String::from(key), Vec::new());
        }
    }
    pub(crate) fn new() -> Env {
        let mut args: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut key = String::new();
        for arg in std::env::args() {
            if let Some(key_new) = arg.strip_prefix("--") {
                key = String::from(key_new);
                Env::ensure_args_key(&mut args, &key);
            } else if let Some(key_new) = arg.strip_prefix('-') {
                key = String::from(key_new);
                Env::ensure_args_key(&mut args, &key);
            } else {
                Env::ensure_args_key(&mut args, &key);
                if let Some(key_args) = args.get_mut(&key) {
                    key_args.push(arg)
                }
            }
        }
        Env { args }
    }
}