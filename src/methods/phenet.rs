use std::fs::File;
use std::io::BufReader;
use crate::data::tsv::TsvReader;
use crate::data::var_id::VarId;
use crate::error::{Error, map_err};
use crate::lang::value::Value;
use std::io::Write;

struct Record {
    var_id: VarId,
    endo_z: f64
}

pub(crate) fn phenet(input: &str, output: &str, z_threshold: f64) -> Result<Value, Error> {
    let tsv_reader =
        TsvReader::from_reader(BufReader::new(
            map_err(File::open(input), input)?
        ))?;
    let mut records: Vec<Record> = Vec::new();
    for row_res in tsv_reader {
        let row = row_res?;
        let mut field_iter = row.iter();
        let var_id =
            VarId::parse(field_iter.next()
                .ok_or_else(|| { Error::from("Empty row") })?)?;
        let endo_z =
            field_iter.next()
                .ok_or_else(||Error::from(format!("Missing Z after {}", var_id)))?
                .parse::<f64>().ok().unwrap_or(f64::NAN);
        if endo_z.abs() >= z_threshold {
            records.push(Record {var_id, endo_z});
        }
    }
    records.sort_by(|r1, r2| r1.var_id.cmp(&r2.var_id));
    let mut out_file = map_err(File::create(output), output)?;
    writeln!(out_file, "Var_ID\tz_endo")?;
    for record in records {
        writeln!(out_file, "{}\t{}", record.var_id, record.endo_z)?;
    }
    Ok(Value::Unit)
}