
pub(crate) struct ColNames {
    names: Vec<String>,
}

fn index_for_raw(names: &[String], name: &str) -> Option<usize> {
    for (i, name_i) in names.iter().enumerate() {
        if name_i.as_str() == name {
            return Some(i);
        }
    }
    None
}

impl ColNames {
    pub(crate) fn new() -> ColNames {
        let names = Vec::new();
        ColNames { names }
    }
    pub(crate) fn index_for(&self, col: &str) -> Result<usize, ErrorOld> {
        match index_for_raw(&self.names, col) {
            Some(i) => {
                Ok(i)
            }
            None => {
                Ok(col.parse::<usize>()?)
            }
        }
    }
}

