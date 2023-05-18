use std::fmt::{Display, Formatter};
use crate::error::Error;

#[derive(Eq, PartialOrd, PartialEq, Ord)]
pub(crate) enum Chromosome {
    Auto(u8),
    Allo(char),
}

impl Chromosome {
    pub(crate) fn parse(string: &str) -> Result<Chromosome, Error> {
        fn chrom_parse_error(string: &str) -> Error {
            Error::from(format!("Cannot parse {} as a chromosome", string))
        }
        match string.parse::<u8>() {
            Ok(num) => { Ok(Chromosome::Auto(num)) }
            Err(_) => {
                if string.len() == 1 {
                    let c = string.chars().next().ok_or_else(|| { chrom_parse_error(string) })?;
                    Ok(Chromosome::Allo(c))
                } else {
                    Err(chrom_parse_error(string))
                }
            }
        }
    }
}

#[derive(Eq, PartialOrd, PartialEq, Ord)]
pub(crate) struct VarId {
    pub(crate) chrom: Chromosome,
    pub(crate) pos: usize,
    pub(crate) seq_ref: String,
    pub(crate) seq_alt: String,
}

impl VarId {
    pub(crate) fn parse(string: &str) -> Result<VarId, Error> {
        fn parse_error(string: &str) -> Error {
            Error::from(format!("Cannot parse {} as a variant id.", string))
        }
        let mut parts = string.split(['_', '/', '-', ':']);
        let chrom =
            Chromosome::parse(parts.next().ok_or_else(|| parse_error(string))?)?;
        let pos = parts.next().ok_or_else(|| parse_error(string))?.parse::<usize>()?;
        let seq_ref = parts.next().ok_or_else(|| parse_error(string))?.to_string();
        let seq_alt = parts.next().ok_or_else(|| parse_error(string))?.to_string();
        Ok(VarId { chrom, pos, seq_ref, seq_alt })
    }
}

impl Display for Chromosome {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Chromosome::Auto(num) => { write!(f, "{}", num) }
            Chromosome::Allo(letter) => { write!(f, "{}", letter) }
        }
    }
}

impl Display for VarId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}_{}_{}", self.chrom, self.pos, self.seq_ref, self.seq_alt)
    }
}